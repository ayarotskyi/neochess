pub mod dto;
pub mod schema;

use std::io;

use anyhow::anyhow;
use async_trait::async_trait;
use diesel::{
    dsl::insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use pgn_reader::Reader;
use thiserror::Error;
use tokio::task;
use uuid::Uuid;

use crate::{
    domain::game::{
        models::game::{CreateGamesError, InvalidPgnError, NewGame},
        ports::GameRepository,
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

    pub async fn save_games(
        &self,
        new_games: Vec<NewGame>,
    ) -> Result<Vec<Uuid>, CreateGamesPostgresError> {
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
                        .on_conflict_do_nothing()
                        .returning(position::id)
                        .get_results(conn)?;

                    if position_ids.len() == 0 {
                        return Err(CreateGamesPostgresError::Unknown(anyhow!(
                            "no position is stored"
                        )));
                    }

                    let game_ids: Vec<uuid::Uuid> = insert_into(game::table)
                        .values(&NewGameDto::from(new_game))
                        .returning(game::id)
                        .get_results(conn)?;

                    if game_ids.len() == 0 {
                        return Err(CreateGamesPostgresError::Unknown(anyhow!(
                            "no game is stored"
                        )));
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

                Ok::<Vec<Uuid>, CreateGamesPostgresError>(resulting_ids)
            })?;

            Ok::<Vec<Uuid>, CreateGamesPostgresError>(game_ids)
        })
        .await?;

        result
    }
}

#[derive(Debug, Error)]
pub enum CreateGamesPostgresError {
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

impl From<CreateGamesPostgresError> for CreateGamesError {
    fn from(value: CreateGamesPostgresError) -> Self {
        match value {
            CreateGamesPostgresError::ConnectionError(err) => {
                CreateGamesError::ConnectionError(err.to_string())
            }
            CreateGamesPostgresError::DieselError(err) => {
                CreateGamesError::DatabaseError(err.to_string())
            }
            err => CreateGamesError::Unknown(anyhow::anyhow!(err)),
        }
    }
}

#[async_trait]
impl GameRepository for Postgres {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, CreateGamesError> {
        Ok(self.save_games(games).await?)
    }
}
