pub mod dto;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pgn_reader::Reader;
use rayon::prelude::*;
use sqlx::{Pool, Row, pool::PoolConnection, postgres::PgRow};
use std::io;
use thiserror::Error;
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::{
    domain::{
        game::{
            models::{
                errors::{GameRepositoryError, InvalidPgnError},
                fen::Fen,
                game::Color,
                move_stat::MoveStat,
                new_game::NewGame,
            },
            ports::GameRepository,
        },
        platform::models::PlatformName,
    },
    outbound::{
        join_set_limited::JoinSetLimited,
        position_visitor::{PositionMetadata, PositionVisitor},
        postgres::dto::{InsertedGameDto, MoveStatDto, NewGameDto},
    },
};
#[derive(Clone)]
struct PositionRelation {
    pub game_id: uuid::Uuid,
    pub metadata: Vec<PositionMetadata>,
}

#[derive(Clone)]
pub struct Postgres {
    pool: Pool<sqlx::Postgres>,
}

impl Postgres {
    pub async fn new(database_url: String) -> anyhow::Result<Self> {
        let pool = Pool::connect(&database_url).await?;

        sqlx::migrate!("src/outbound/postgres/migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    fn games_to_bytes(game_dto_vec: Vec<NewGameDto>) -> Result<Vec<u8>, PostgresError> {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = pgcopy::Encoder::new(&mut buf);
        encoder.write_header().unwrap();

        for game_dto in game_dto_vec {
            encoder.write_tuple(8)?;
            encoder.write_str(game_dto.white)?;
            encoder.write_smallint(game_dto.white_elo)?;
            encoder.write_str(game_dto.black)?;
            encoder.write_smallint(game_dto.black_elo)?;
            match game_dto.winner {
                Some(winner) => encoder.write_str(winner)?,
                None => encoder.write_null()?,
            };
            encoder.write_str(game_dto.platform_name)?;
            encoder.write_str(game_dto.pgn)?;
            encoder.write_timestamp_with_time_zone(game_dto.finished_at)?;
        }

        encoder.write_trailer()?;

        return Ok(buf);
    }

    fn position_relation_vec_to_bytes(
        position_relation_vec: Vec<PositionRelation>,
    ) -> Result<Vec<u8>, PostgresError> {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = pgcopy::Encoder::new(&mut buf);
        encoder.write_header().unwrap();

        for position_relation in position_relation_vec {
            for (move_idx, position_meta) in position_relation.metadata.iter().enumerate() {
                encoder.write_tuple(4)?;
                encoder.write_uuid(*position_relation.game_id.as_bytes())?;
                encoder.write_smallint(move_idx as i16)?;
                encoder.write_str(position_meta.fen.to_string())?;
                match position_meta.next_move_uci {
                    Some(uci) => encoder.write_str(uci.to_string())?,
                    None => encoder.write_null()?,
                };
            }
        }

        encoder.write_trailer()?;

        return Ok(buf);
    }

    async fn copy_games(
        new_game_dto_vec: Vec<NewGameDto>,
        temp_table_name: String,
        mut conn: PoolConnection<sqlx::Postgres>,
    ) -> Result<(), PostgresError> {
        let mut copy_in = conn
            .copy_in_raw(&format!(
                "COPY \"{}\" 
        (white, white_elo, black, black_elo, winner, platform_name, pgn, finished_at) 
        FROM STDIN 
        WITH (FORMAT binary);",
                temp_table_name
            ))
            .await?;
        let result = copy_in.send(Self::games_to_bytes(new_game_dto_vec)?).await;

        match result {
            Ok(_) => {
                copy_in.finish().await?;
            }
            Err(_) => {
                copy_in.abort("").await?;
            }
        }

        Ok(())
    }

    async fn copy_positions(
        inserted_games: Vec<InsertedGameDto>,
        mut conn: PoolConnection<sqlx::Postgres>,
    ) -> Result<(), PostgresError> {
        let position_relation_vec: Vec<PositionRelation> = inserted_games
            .par_iter()
            .filter_map(|inserted_game| {
                let mut reader = Reader::new(io::Cursor::new(&inserted_game.pgn));
                let metadata = match reader.read_game(&mut PositionVisitor::new(&inserted_game.pgn))
                {
                    Ok(option) => match option {
                        Some(result) => match result {
                            Ok(result) => result,
                            Err(_) => return None,
                        },
                        None => return None,
                    },
                    Err(_) => return None,
                };

                return Some(PositionRelation {
                    game_id: inserted_game.id,
                    metadata,
                });
            })
            .collect::<Vec<_>>();

        let mut copy_in = conn
            .copy_in_raw(
                "COPY game_position 
        (game_id, move_idx, fen, next_move_uci) 
        FROM STDIN 
        WITH (FORMAT binary);",
            )
            .await?;

        let result = copy_in
            .send(Self::position_relation_vec_to_bytes(position_relation_vec)?)
            .await;
        match result {
            Ok(_) => {
                copy_in.finish().await?;
            }
            Err(_) => {
                copy_in.abort("").await?;
            }
        }

        Ok(())
    }

    async fn save_games_distributed(
        &self,
        new_games: Vec<NewGame>,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Vec<Uuid>, PostgresError> {
        let games_temp_table_name = format!(
            "temp_game_{}_{}",
            username,
            Into::<&'static str>::into(platform_name)
        );

        sqlx::query(&format!(
            "DROP TABLE IF EXISTS \"{}\";",
            games_temp_table_name
        ))
        .execute(&self.pool)
        .await?;

        sqlx::query(&format!(
            "CREATE UNLOGGED TABLE IF NOT EXISTS \"{}\" (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            white VARCHAR NOT NULL,
            white_elo SMALLINT NOT NULL,
            black VARCHAR NOT NULL,
            black_elo SMALLINT NOT NULL,
            winner CHAR(5),
            platform_name VARCHAR NOT NULL,
            pgn VARCHAR NOT NULL,
            finished_at TIMESTAMP WITH TIME ZONE NOT NULL,
            UNIQUE (white, black, finished_at, platform_name)
        );",
            games_temp_table_name
        ))
        .execute(&self.pool)
        .await?;

        let mut join_set = JoinSet::new();
        new_games
            .chunks(new_games.len() / 8)
            .for_each(|new_games_chunk| {
                let pool = self.pool.clone();
                let new_game_dto_vec = new_games_chunk
                    .iter()
                    .map(|new_game| new_game.clone().into())
                    .collect::<_>();
                let games_temp_table_name = games_temp_table_name.clone();

                join_set.spawn(async move {
                    let conn = pool.acquire().await?;
                    let res = Self::copy_games(new_game_dto_vec, games_temp_table_name, conn).await;
                    res
                });
            });

        while let Some(result) = join_set.join_next().await {
            result??;
        }

        let inserted_games: Vec<InsertedGameDto> = sqlx::query_as(&format!(
            "INSERT INTO game
        SELECT * FROM \"{}\"
        ON CONFLICT DO NOTHING
        RETURNING id, pgn, finished_at",
            games_temp_table_name
        ))
        .fetch_all(&self.pool)
        .await?;

        let connections_limit = 8;
        let mut join_set = JoinSetLimited::new(
            inserted_games.chunks(300).map(|inserted_games_chunk| {
                let pool = self.pool.clone();
                let inserted_games = inserted_games_chunk.to_vec();

                async move {
                    let conn = pool.acquire().await?;
                    Self::copy_positions(inserted_games, conn).await
                }
            }),
            connections_limit,
        );

        while let Some(result) = join_set.join_next().await {
            result??;
        }

        sqlx::query(&format!(
            "DROP TABLE IF EXISTS \"{}\";",
            games_temp_table_name
        ))
        .execute(&self.pool)
        .await?;

        return Ok(inserted_games
            .iter()
            .map(|inserted_game| inserted_game.id)
            .collect::<_>());
    }

    async fn latest_game_timestamp_seconds_by_username(
        &self,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, PostgresError> {
        let latest_timestamp: Option<DateTime<Utc>> = sqlx::query(
            "SELECT MAX(finished_at) FROM game
        WHERE platform_name = $1
        AND (
            white = $2
            OR black = $2
        )",
        )
        .bind(Into::<&'static str>::into(platform_name))
        .bind(username)
        .map(|row: PgRow| row.get(0))
        .fetch_one(&self.pool)
        .await?;

        return Ok(latest_timestamp);
    }

    pub async fn query_move_stats(
        &self,
        position_fen: &Fen,
        username: &str,
        play_as: &Color,
        platform_name: &PlatformName,
        from_timestamp_seconds: &Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp_seconds: &Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, PostgresError> {
        let move_stats_dto: Vec<MoveStatDto> = sqlx::query_as(&format!(
            "SELECT game_position.next_move_uci,
                    COUNT(*) total,
                    SUM(case when game.winner = $1 then 1 else 0 end) wins,
                    SUM(case when game.winner is NULL then 1 else 0 end) draws,
                    AVG({})::INT avg_opponent_elo,
                    MAX(game.finished_at) last_played_at
                FROM game_position
                    JOIN game ON game.id = game_position.game_id
                WHERE game.platform_name = $2
                    AND game_position.fen = $3
                    AND game_position.next_move_uci IS NOT NULL
                    AND {} = $4
                    AND ($5 is NULL OR game.finished_at >= $5)
                    AND ($6 is NULL OR game.finished_at <= $6)
                GROUP BY game_position.next_move_uci",
            match play_as {
                Color::White => "game.black_elo",
                Color::Black => "game.white_elo",
            },
            match play_as {
                Color::White => "game.white",
                Color::Black => "game.black",
            }
        ))
        .bind(Into::<&'static str>::into(play_as))
        .bind(Into::<&'static str>::into(platform_name))
        .bind(position_fen.to_string())
        .bind(username)
        .bind(from_timestamp_seconds)
        .bind(to_timestamp_seconds)
        .fetch_all(&self.pool)
        .await?;

        Ok(move_stats_dto
            .into_iter()
            .map(|move_stat_dto| move_stat_dto.into())
            .collect::<_>())
    }
}

#[derive(Debug, Error)]
pub enum PostgresError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    InvalidPgn(#[from] InvalidPgnError),
    #[error(transparent)]
    SerializationError(#[from] std::io::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<PostgresError> for GameRepositoryError {
    fn from(value: PostgresError) -> Self {
        match value {
            PostgresError::DatabaseError(err) => {
                GameRepositoryError::DatabaseError(err.to_string())
            }
            err => GameRepositoryError::Unknown(anyhow::anyhow!(err)),
        }
    }
}

#[async_trait]
impl GameRepository for Postgres {
    async fn store_games(
        &self,
        games: Vec<NewGame>,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Vec<Uuid>, GameRepositoryError> {
        if games.len() == 0 {
            return Ok(Vec::new());
        }

        Ok(self
            .save_games_distributed(games, platform_name, username)
            .await?)
    }

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Option<u64>, GameRepositoryError> {
        let timestamp = self
            .latest_game_timestamp_seconds_by_username(platform_name, username)
            .await?;

        Ok(timestamp.map(|ts| (ts.timestamp_millis() / 1000) as u64))
    }

    async fn get_move_stats(
        &self,
        position_fen: &Fen,
        username: &str,
        play_as: &Color,
        platform_name: &PlatformName,
        from_timestamp: &Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp: &Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError> {
        Ok(self
            .query_move_stats(
                position_fen,
                username,
                play_as,
                platform_name,
                from_timestamp,
                to_timestamp,
            )
            .await?)
    }
}
