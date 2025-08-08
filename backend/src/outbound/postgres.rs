pub mod dto;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pgn_reader::Reader;
use rayon::prelude::*;
use sqlx::{Pool, Row, pool::PoolConnection, postgres::PgRow};
use std::{
    io,
    sync::{Arc, atomic::AtomicI16},
    time::Instant,
};
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
        position_visitor::{PositionMetadata, PositionVisitor},
        postgres::dto::{MoveStatDto, NewGameDto},
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
        copy_in
            .send(Self::games_to_bytes(new_game_dto_vec)?)
            .await?;
        copy_in.finish().await?;

        Ok(())
    }

    async fn copy_positions(
        position_relation_vec: Vec<PositionRelation>,
        temp_table_name: String,
        mut conn: PoolConnection<sqlx::Postgres>,
    ) -> Result<(), PostgresError> {
        let mut copy_in = conn
            .copy_in_raw(&format!(
                "COPY \"{}\" 
        (game_id, move_idx, fen, next_move_uci) 
        FROM STDIN 
        WITH (FORMAT binary);",
                temp_table_name
            ))
            .await?;

        copy_in
            .send(Self::position_relation_vec_to_bytes(position_relation_vec)?)
            .await?;
        copy_in.finish().await?;

        Ok(())
    }

    async fn save_games_distributed(
        &self,
        new_games: Vec<NewGame>,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Vec<Uuid>, PostgresError> {
        let instant = Instant::now();

        let games_temp_table_name = format!(
            "temp_game_{}_{}",
            username,
            Into::<&'static str>::into(platform_name)
        );
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

        println!("created temp table in {:?}", instant.elapsed());

        let instant = Instant::now();
        let mut join_set = JoinSet::new();
        let count = Arc::new(AtomicI16::new(0));
        let amount = new_games.len() / 1000;
        new_games.chunks(1000).for_each(|new_games_chunk| {
            let pool = self.pool.clone();
            let new_game_dto_vec = new_games_chunk
                .iter()
                .map(|new_game| new_game.clone().into())
                .collect::<_>();
            let games_temp_table_name = games_temp_table_name.clone();

            let count = count.clone();
            join_set.spawn(async move {
                let conn = pool.acquire().await?;
                let res = Self::copy_games(new_game_dto_vec, games_temp_table_name, conn).await;
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!("finished copying games {:?} / {}", count, amount);
                res
            });
        });

        println!("created tasks for copying games in {:?}", instant.elapsed());

        let instant = Instant::now();
        while let Some(result) = join_set.join_next().await {
            result??;
        }
        println!("finished copying games in {:?}", instant.elapsed());

        let instant = Instant::now();
        let game_entries: Vec<(Uuid, String, DateTime<Utc>)> = sqlx::query(&format!(
            "INSERT INTO game
        SELECT * FROM \"{}\"
        ON CONFLICT DO NOTHING
        RETURNING id, pgn, finished_at",
            games_temp_table_name
        ))
        .map(|row: PgRow| (row.get("id"), row.get("pgn"), row.get("finished_at")))
        .fetch_all(&self.pool)
        .await?;
        println!(
            "finished inserting games from temp table in {:?}",
            instant.elapsed()
        );

        let instant = Instant::now();
        let position_relation_vec: Vec<PositionRelation> = game_entries
            .par_iter()
            .filter_map(|(game_id, pgn, _)| {
                let mut reader = Reader::new(io::Cursor::new(pgn));
                let metadata = match reader.read_game(&mut PositionVisitor::new(pgn)) {
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
                    game_id: *game_id,
                    metadata,
                });
            })
            .collect::<Vec<_>>();
        println!("{}", position_relation_vec.len());
        println!(
            "finished creating position relation vector in {:?}",
            instant.elapsed()
        );

        let instant = Instant::now();
        let position_relation_temp_table_name = format!(
            "temp_position_relation_{}_{}",
            username,
            Into::<&'static str>::into(platform_name)
        );
        sqlx::query(&format!(
            "CREATE UNLOGGED TABLE IF NOT EXISTS \"{}\" (
            game_id UUID NOT NULL,
            move_idx SMALLINT,
            fen TEXT NOT NULL,
            next_move_uci TEXT
        );",
            position_relation_temp_table_name
        ))
        .execute(&self.pool)
        .await?;
        println!("created temp position table in {:?}", instant.elapsed());

        let instant = Instant::now();
        let mut join_set = JoinSet::new();
        let count = Arc::new(AtomicI16::new(0));
        let amount = position_relation_vec.len() / 50;
        position_relation_vec
            .chunks(50)
            .for_each(|position_relation_chunk| {
                let pool = self.pool.clone();
                let position_relation_vec = position_relation_chunk.to_vec().clone();
                let position_relation_temp_table_name = position_relation_temp_table_name.clone();

                let count = count.clone();
                join_set.spawn(async move {
                    let conn = pool.acquire().await?;
                    let res = Self::copy_positions(
                        position_relation_vec,
                        position_relation_temp_table_name,
                        conn,
                    )
                    .await;
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    println!("finished copying positions {:?} / {}", count, amount);
                    res
                });
            });
        println!(
            "finished creating position copying tasks in {:?}",
            instant.elapsed()
        );

        let instant = Instant::now();
        while let Some(result) = join_set.join_next().await {
            result??;
        }
        println!("finished copying positions in {:?}", instant.elapsed());

        let instant = Instant::now();
        sqlx::query(&format!(
            "INSERT INTO position (fen)
            SELECT fen FROM \"{}\"
            ON CONFLICT DO NOTHING;",
            position_relation_temp_table_name
        ))
        .execute(&self.pool)
        .await?;
        println!("finished inserting position in {:?}", instant.elapsed());

        let instant = Instant::now();
        sqlx::query(&format!(
            "INSERT INTO game_position (game_id, position_id, move_idx, next_move_uci)
            SELECT game_id, position.id, move_idx, next_move_uci FROM \"{0}\"
            INNER JOIN position ON position.fen = \"{0}\".fen
            ON CONFLICT DO NOTHING;",
            position_relation_temp_table_name
        ))
        .execute(&self.pool)
        .await?;
        println!(
            "finished inserting game_position in {:?}",
            instant.elapsed()
        );

        sqlx::query(&format!(
            "DROP TABLE IF EXISTS \"{}\";",
            games_temp_table_name
        ))
        .execute(&self.pool)
        .await?;

        sqlx::query(&format!(
            "DROP TABLE IF EXISTS \"{}\";",
            position_relation_temp_table_name
        ))
        .execute(&self.pool)
        .await?;

        return Ok(game_entries
            .iter()
            .map(|game_entry| game_entry.0)
            .collect::<_>());
    }

    async fn latest_game_timestamp_seconds_by_username(
        &self,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, PostgresError> {
        let record = sqlx::query!(
            "SELECT MAX(finished_at) FROM game
        WHERE platform_name = $1
        AND (
            white = $2
            OR black = $2
        )",
            Into::<&'static str>::into(platform_name),
            username
        )
        .fetch_one(&self.pool)
        .await?;

        return Ok(record.max);
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
