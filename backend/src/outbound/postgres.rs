pub mod dto;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pgn_reader::Reader;
use rayon::prelude::*;
use sqlx::{Pool, QueryBuilder, Row};
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

    pub async fn save_games(&self, new_games: Vec<NewGame>) -> Result<Vec<Uuid>, PostgresError> {
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

        let mut game_entries: Vec<(Uuid, DateTime<Utc>)> = Vec::with_capacity(games_len);

        for game_chunk in game_map
            .values()
            .collect::<Vec<_>>()
            // Each game entry requires 8 params
            .chunks((u16::MAX / 8).into())
        {
            let mut game_query_builder = QueryBuilder::new(
                "INSERT INTO game 
                (white, white_elo, black, black_elo, winner, platform_name, pgn, finished_at) ",
            );

            game_query_builder.push_values(game_chunk, |mut b, (new_game_dto, _)| {
                b.push_bind(new_game_dto.white.clone())
                    .push_bind(new_game_dto.white_elo)
                    .push_bind(new_game_dto.black.clone())
                    .push_bind(new_game_dto.black_elo)
                    .push_bind(new_game_dto.winner.clone())
                    .push_bind(new_game_dto.platform_name.clone())
                    .push_bind(new_game_dto.pgn.clone())
                    .push_bind(new_game_dto.finished_at);
            });

            game_query_builder.push(
                " ON CONFLICT DO NOTHING
        RETURNING id, finished_at;",
            );

            let game_rows = game_query_builder.build().fetch_all(&mut *tx).await?;
            for row in game_rows {
                let id: Uuid = row.try_get("id")?;
                let finished_at: DateTime<Utc> = row.try_get("finished_at")?;
                game_entries.push((id, finished_at));
            }
        }

        let position_meta_vec: Vec<(usize, Uuid, PositionMetadata)> = game_entries
            .par_iter()
            .filter_map(|(game_id, finished_at)| {
                game_map.get(&finished_at).map(|(_, meta_vec)| {
                    meta_vec
                        .iter()
                        .enumerate()
                        .map(|(move_idx, position_meta)| {
                            (move_idx, *game_id, position_meta.clone())
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>()
            .concat();

        // Split query into chunks with size u16::MAX / 4
        // Maximum amount of arguments in postgres is u16::MAX
        // This query uses 4 arguments per one metadata entry
        for position_meta_chunk in position_meta_vec.chunks((u16::MAX / 4).into()) {
            let mut position_query_builder: QueryBuilder<'_, sqlx::Postgres> = QueryBuilder::new(
                "WITH meta AS (
                SELECT * FROM (",
            );

            position_query_builder.push_values(
                position_meta_chunk,
                |mut b, (move_idx, game_id, position_meta)| {
                    b.push_bind(position_meta.fen.to_string())
                        .push_bind(position_meta.next_move_uci.map(|uci| uci.to_string()))
                        .push_bind(*move_idx as i64)
                        .push_bind(game_id);
                },
            );

            position_query_builder.push(
                ") as t(fen, next_move_uci, move_idx, game_id)
            ), inserted_position AS (INSERT INTO position (fen)
                SELECT DISTINCT(meta.fen) FROM meta
                ON CONFLICT (fen) DO UPDATE
                SET fen = EXCLUDED.fen
                RETURNING position.id, position.fen)
            INSERT INTO game_position
                SELECT meta.game_id::UUID as game_id, 
                inserted_position.id as position_id, 
                meta.move_idx as move_idx, 
                meta.next_move_uci as next_move_uci
                FROM inserted_position
                INNER JOIN meta ON inserted_position.fen = meta.fen
            ON CONFLICT DO NOTHING;",
            );

            position_query_builder.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;

        Ok::<Vec<Uuid>, PostgresError>(
            game_entries
                .iter()
                .map(|(game_id, _)| *game_id)
                .collect::<_>(),
        )
    }

    pub async fn latest_game_timestamp_seconds_by_username(
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
        Ok(self.save_games(games).await?)
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
