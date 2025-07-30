pub mod dto;
pub mod schema;

use std::io;

use anyhow::anyhow;
use async_trait::async_trait;
use diesel::{
    dsl::insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    sql_query,
    upsert::excluded,
};
use pgn_reader::Reader;
use thiserror::Error;
use tokio::task;
use uuid::Uuid;

use crate::{
    domain::{
        game::{
            models::{
                game::{Color, GameRepositoryError, InvalidPgnError, NewGame},
                position::{Fen, MoveStat},
            },
            ports::GameRepository,
        },
        platform::models::PlatformName,
    },
    outbound::{
        position_visitor::PositionVisitor,
        postgres::dto::{GamePositionDto, MoveStatDto, NewGameDto, NewPositionDto},
    },
};

#[derive(Clone)]
pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        Self {
            pool: Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .expect("Could not build connection pool"),
        }
    }

    pub async fn save_games(&self, new_games: Vec<NewGame>) -> Result<Vec<Uuid>, PostgresError> {
        let pool = self.pool.clone();

        let result = task::spawn_blocking(move || {
            use schema::game;
            use schema::game_position;
            use schema::position;

            let mut conn = pool.get()?;

            let game_ids = conn.transaction(|conn| {
                let mut resulting_ids: Vec<Uuid> = Vec::with_capacity(new_games.len());
                for new_game in new_games {
                    let mut reader = Reader::new(io::Cursor::new(new_game.pgn()));

                    let position_metadata_vec = reader
                        .read_game(&mut PositionVisitor::new(new_game.pgn().into()))
                        .map_err(|_| InvalidPgnError(new_game.pgn().into()))?
                        .unwrap_or(Err(InvalidPgnError(new_game.pgn().into())))?;

                    let position_ids: Vec<uuid::Uuid> = insert_into(position::table)
                        .values(
                            &position_metadata_vec
                                .iter()
                                .map(|metadata| NewPositionDto {
                                    fen: metadata.fen.to_string(),
                                })
                                .collect::<Vec<NewPositionDto>>(),
                        )
                        .on_conflict(position::columns::fen)
                        .do_update()
                        .set(position::columns::fen.eq(excluded(position::columns::fen)))
                        .returning(position::id)
                        .get_results(conn)?;

                    if position_ids.len() != position_metadata_vec.len() {
                        return Err(PostgresError::Unknown(anyhow!(
                            "expected to store {} positions, but got {}",
                            position_metadata_vec.len(),
                            position_ids.len()
                        )));
                    }

                    let game_result: Option<uuid::Uuid> = insert_into(game::table)
                        .values(&NewGameDto::from(new_game))
                        .returning(game::id)
                        .on_conflict_do_nothing()
                        .get_result(conn)
                        .optional()?;

                    let game_id = match game_result {
                        Some(id) => id,
                        None => {
                            // If the game already exists, we skip inserting positions
                            continue;
                        }
                    };

                    insert_into(game_position::table)
                        .values(
                            &position_ids
                                .into_iter()
                                .enumerate()
                                .map(|(index, position_id)| GamePositionDto {
                                    game_id: game_id,
                                    position_id: position_id,
                                    move_idx: index as i16,
                                    next_move_san: position_metadata_vec.get(index).and_then(
                                        |metadata| metadata.next_move_san.map(|s| s.to_string()),
                                    ),
                                })
                                .collect::<Vec<GamePositionDto>>(),
                        )
                        .execute(conn)?;

                    resulting_ids.push(game_id);
                }

                Ok::<Vec<Uuid>, PostgresError>(resulting_ids)
            })?;

            Ok::<Vec<Uuid>, PostgresError>(game_ids)
        })
        .await?;

        result
    }

    pub async fn latest_game_timestamp_seconds_by_username(
        &self,
        platform_name: String,
        username: String,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, PostgresError> {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            use schema::game;

            let mut conn = pool.get()?;
            game::dsl::game
                .filter(schema::game::columns::platform_name.eq(platform_name))
                .filter(
                    schema::game::columns::white
                        .eq(username.clone())
                        .or(schema::game::columns::black.eq(username.clone())),
                )
                .select(diesel::dsl::max(game::columns::finished_at))
                .first::<Option<chrono::DateTime<chrono::Utc>>>(&mut conn)
                .map_err(|e| PostgresError::DieselError(e))
        })
        .await
        .map_err(|e| PostgresError::JoinError(e))?
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
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            use schema::game;

            let mut conn = pool.get()?;

            let query_str = format!(
                "SELECT game_position.next_move_san,
                    COUNT(*) total,
                    SUM(case when game.winner = $1 then 1 else 0 end) wins,
                    SUM(case when game.winner is NULL then 1 else 0 end) draws,
                    AVG({})::INT avg_opponent_elo
                FROM game_position
                    JOIN position ON position.id = game_position.position_id
                    JOIN game ON game.id = game_position.game_id
                WHERE game.platform_name = $2
                    AND position.fen = $3
                    AND {} = $4
                GROUP BY game_position.next_move_san",
                match play_as {
                    Color::White => game::black_elo::NAME,
                    Color::Black => game::white_elo::NAME,
                },
                match play_as {
                    Color::White => game::white::NAME,
                    Color::Black => game::black::NAME,
                }
            );

            let query_result: Vec<MoveStatDto> = sql_query(query_str)
                .bind::<diesel::sql_types::Text, _>(Into::<&'static str>::into(play_as))
                .bind::<diesel::sql_types::Text, _>(Into::<&'static str>::into(platform_name))
                .bind::<diesel::sql_types::Text, _>(position_fen.to_string())
                .bind::<diesel::sql_types::Text, _>(username)
                // .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(
                //     from_timestamp_seconds,
                // )
                // .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(
                //     to_timestamp_seconds,
                // )
                .get_results(&mut conn)?;

            let move_stats = query_result
                .into_iter()
                .map(|move_stat_dto| {
                    MoveStat::new(
                        move_stat_dto.next_move_san,
                        move_stat_dto.total as u64,
                        move_stat_dto.wins as u64,
                        move_stat_dto.draws as u64,
                        move_stat_dto.avg_opponent_elo as u8,
                    )
                })
                .collect::<_>();

            Ok(move_stats)
        })
        .await?
    }
}

#[derive(Debug, Error)]
pub enum PostgresError {
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    #[error(transparent)]
    ConnectionError(#[from] r2d2::Error),
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
            PostgresError::ConnectionError(err) => {
                GameRepositoryError::ConnectionError(err.to_string())
            }
            PostgresError::DieselError(err) => GameRepositoryError::DatabaseError(err.to_string()),
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
