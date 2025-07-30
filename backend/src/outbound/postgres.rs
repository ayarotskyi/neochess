pub mod dto;
pub mod schema;

use std::io;

use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{
    dsl::insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
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
                position::MoveStat,
            },
            ports::GameRepository,
        },
        platform::models::PlatformName,
    },
    outbound::{
        position_visitor::PositionVisitor,
        postgres::dto::{GamePositionDto, NewGameDto, NewPositionDto},
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

                    let game_ids: Vec<uuid::Uuid> = insert_into(game::table)
                        .values(&NewGameDto::from(new_game))
                        .returning(game::id)
                        .get_results(conn)?;

                    if game_ids.len() == 0 {
                        return Err(PostgresError::Unknown(anyhow!("no game is stored")));
                    }

                    let game_id = game_ids[0];

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

    pub async fn latest_game_timestamp_by_username(
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
        position_fen: String,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp: Option<i32>,
        to_timestamp: Option<i32>,
    ) -> Result<Vec<MoveStat>, PostgresError> {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            use schema::game;
            use schema::game_position;
            use schema::position;

            let mut conn = pool.get()?;

            let mut query = game::dsl::game
                .inner_join(
                    game_position::dsl::game_position
                        .on(game::columns::id.eq(game_position::columns::game_id)),
                )
                .inner_join(
                    position::dsl::position
                        .on(game_position::columns::position_id.eq(position::columns::id)),
                )
                .filter(
                    game::columns::platform_name
                        .eq(Into::<&'static str>::into(platform_name).to_string()),
                )
                .filter(position::columns::fen.eq(position_fen))
                .into_boxed();

            if play_as == Color::White {
                query = query.filter(game::columns::white.eq(username));
            } else {
                query = query.filter(game::columns::black.eq(username));
            }

            if let Some(from) = from_timestamp {
                query = query.filter(
                    game::columns::finished_at.ge(chrono::DateTime::<chrono::Utc>::from_timestamp(
                        from as i64,
                        0,
                    )
                    .unwrap_or(DateTime::UNIX_EPOCH)),
                );
            }

            if let Some(to) = to_timestamp {
                query = query.filter(
                    game::columns::finished_at.le(chrono::DateTime::<chrono::Utc>::from_timestamp(
                        to as i64, 0,
                    )
                    .unwrap_or(Utc::now())),
                );
            }

            Ok(Vec::new())

            // let result: Vec<(String, i8, String, i8, i16, String)> = query.load(&mut conn)?;

            // Ok(result
            //     .into_iter()
            //     .map(|(id, white, white_elo, black, black_elo, move_idx, fen)| {
            //         MoveStat::new(
            //             white,
            //             white_elo,
            //             black,
            //             black_elo,
            //             move_idx as usize,
            //             fen,
            //         )
            //     })
            //     .collect())
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

    async fn get_latest_game_timestamp(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError> {
        let timestamp = self
            .latest_game_timestamp_by_username(
                Into::<&'static str>::into(platform_name).to_string(),
                username,
            )
            .await?;

        Ok(timestamp.map(|ts| ts.timestamp_millis() as u64))
    }

    async fn get_move_stats(
        &self,
        position_fen: String,
        username: String,
        platform_name: PlatformName,
        from_timestamp: Option<i32>,
        to_timestamp: Option<i32>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError> {
        Ok(self
            .query_move_stats(
                position_fen,
                username,
                Color::White, // Assuming the color is White for this example
                platform_name,
                from_timestamp,
                to_timestamp,
            )
            .await?)
    }
}
