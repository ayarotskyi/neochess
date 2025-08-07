pub mod dto;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pgn_reader::Reader;
use rayon::prelude::*;
use sqlx::{Pool, Row, postgres::PgRow};
use std::{collections::HashMap, io};
use thiserror::Error;
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
        position_visitor::{PositionMetadata, PositionVisitor},
        postgres::dto::{MoveStatDto, NewGameDto},
    },
};
#[derive(Clone)]
struct PositionRelation {
    pub move_idx: usize,
    pub game_id: uuid::Uuid,
    pub metadata: PositionMetadata,
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

    fn games_to_bytes(&self, game_dto_vec: Vec<NewGameDto>) -> Result<Vec<u8>, PostgresError> {
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
        &self,
        position_relation_vec: Vec<PositionRelation>,
    ) -> Result<Vec<u8>, PostgresError> {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = pgcopy::Encoder::new(&mut buf);
        encoder.write_header().unwrap();

        for position_relation in position_relation_vec {
            encoder.write_tuple(4)?;
            encoder.write_uuid(*position_relation.game_id.as_bytes())?;
            encoder.write_smallint(position_relation.move_idx as i16)?;
            encoder.write_str(position_relation.metadata.fen.to_string())?;
            match position_relation.metadata.next_move_uci {
                Some(uci) => encoder.write_str(uci.to_string())?,
                None => encoder.write_null()?,
            };
        }

        encoder.write_trailer()?;

        return Ok(buf);
    }

    async fn save_games(&self, new_games: Vec<NewGame>) -> Result<Vec<Uuid>, PostgresError> {
        let games_len = new_games.len();

        if games_len == 0 {
            return Ok(Vec::new());
        }

        let game_map: HashMap<DateTime<Utc>, (NewGameDto, Vec<PositionMetadata>)> =
            HashMap::from_par_iter(new_games.into_par_iter().filter_map(|new_game| {
                let mut reader = Reader::new(io::Cursor::new(new_game.pgn()));
                let position_meta_vec =
                    match reader.read_game(&mut PositionVisitor::new(new_game.pgn().into())) {
                        Ok(option) => match option {
                            Some(result) => match result {
                                Ok(result) => result,
                                Err(_) => return None,
                            },
                            None => return None,
                        },
                        Err(_) => return None,
                    };

                Some((
                    *new_game.finished_at(),
                    (new_game.into(), position_meta_vec),
                ))
            }));

        let mut tx = self.pool.begin().await?;

        let temp_game_table_name = Uuid::new_v4();

        sqlx::query(&format!(
            "CREATE TEMPORARY TABLE \"{}\" (
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
            temp_game_table_name
        ))
        .execute(&mut *tx)
        .await?;

        let mut copy_in = tx
            .copy_in_raw(&format!(
                "COPY \"{}\" 
        (white, white_elo, black, black_elo, winner, platform_name, pgn, finished_at) 
        FROM STDIN 
        WITH (FORMAT binary);",
                temp_game_table_name
            ))
            .await?;
        copy_in
            .send(
                self.games_to_bytes(
                    game_map
                        .values()
                        .map(|(game_dto, _)| game_dto.clone())
                        .collect::<_>(),
                )?,
            )
            .await?;
        copy_in.finish().await?;

        let game_entries: Vec<(Uuid, DateTime<Utc>)> = sqlx::query(&format!(
            "INSERT INTO game
        SELECT * FROM \"{}\"
        ON CONFLICT DO NOTHING
        RETURNING id, finished_at;",
            temp_game_table_name
        ))
        .map(|row: PgRow| (row.get("id"), row.get("finished_at")))
        .fetch_all(&mut *tx)
        .await?;

        let position_relation_vec: Vec<PositionRelation> = game_entries
            .par_iter()
            .filter_map(|(game_id, finished_at)| {
                game_map.get(&finished_at).map(|(_, meta_vec)| {
                    meta_vec
                        .iter()
                        .enumerate()
                        .map(|(move_idx, position_meta)| PositionRelation {
                            move_idx,
                            game_id: *game_id,
                            metadata: position_meta.clone(),
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>()
            .concat();

        let position_relation_table_name = Uuid::new_v4();

        sqlx::query(&format!(
            "CREATE TEMPORARY TABLE \"{}\" (
            game_id UUID NOT NULL,
            move_idx SMALLINT,
            fen TEXT NOT NULL,
            next_move_uci TEXT
        );",
            position_relation_table_name
        ))
        .execute(&mut *tx)
        .await?;

        let mut copy_in = tx
            .copy_in_raw(&format!(
                "COPY \"{}\" 
        (game_id, move_idx, fen, next_move_uci) 
        FROM STDIN 
        WITH (FORMAT binary);",
                position_relation_table_name
            ))
            .await?;

        copy_in
            .send(self.position_relation_vec_to_bytes(position_relation_vec)?)
            .await?;
        copy_in.finish().await?;

        sqlx::query(&format!(
            "INSERT INTO position (fen)
            SELECT fen FROM \"{}\"
            ON CONFLICT DO NOTHING;",
            position_relation_table_name
        ))
        .execute(&mut *tx)
        .await?;

        sqlx::query(&format!(
            "INSERT INTO game_position (game_id, position_id, move_idx, next_move_uci)
            SELECT game_id, position.id, move_idx, next_move_uci FROM \"{0}\"
            INNER JOIN position ON position.fen = \"{0}\".fen
            ON CONFLICT DO NOTHING;",
            position_relation_table_name
        ))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        println!("finished storing");

        Ok::<Vec<Uuid>, PostgresError>(
            game_entries
                .iter()
                .map(|(game_id, _)| *game_id)
                .collect::<_>(),
        )
    }

    async fn save_games_distributed(
        &self,
        new_games: Vec<NewGame>,
    ) -> Result<Vec<Uuid>, PostgresError> {
        let tasks = new_games
            .chunks(1000)
            .map(|new_games_chunk| self.save_games(new_games_chunk.to_vec()));

        let mut result = Vec::new();
        let mut count = 0;
        let length = tasks.len();
        for task in tasks {
            result.extend(task.await?);
            count = count + 1;
            println!("finished task {}/{}", count, length);
        }

        return Ok(result);
    }

    async fn latest_game_timestamp_seconds_by_username(
        &self,
        platform_name: String,
        username: String,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, PostgresError> {
        let record = sqlx::query!(
            "SELECT MAX(finished_at) FROM game 
        WHERE platform_name = $1
        AND (
            white = $2
            OR black = $2
        )",
            platform_name,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(record.max);
    }

    pub async fn query_move_stats(
        &self,
        position_fen: Fen,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp_seconds: Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp_seconds: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, PostgresError> {
        let move_stats_dto: Vec<MoveStatDto> = sqlx::query_as(&format!(
            "SELECT game_position.next_move_uci,
                    COUNT(*) total,
                    SUM(case when game.winner = $1 then 1 else 0 end) wins,
                    SUM(case when game.winner is NULL then 1 else 0 end) draws,
                    AVG({})::INT avg_opponent_elo,
                    MAX(game.finished_at) last_played_at
                FROM game_position
                    JOIN position ON position.id = game_position.position_id
                    JOIN game ON game.id = game_position.game_id
                WHERE game.platform_name = $2
                    AND position.fen = $3
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
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError> {
        Ok(self.save_games_distributed(games).await?)
    }

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError> {
        let timestamp = self
            .latest_game_timestamp_seconds_by_username(
                Into::<&'static str>::into(platform_name).to_string(),
                username,
            )
            .await?;

        Ok(timestamp.map(|ts| (ts.timestamp_millis() / 1000) as u64))
    }

    async fn get_move_stats(
        &self,
        position_fen: Fen,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime<chrono::Utc>>,
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
