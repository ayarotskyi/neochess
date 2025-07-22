pub mod dto;
pub mod schema;

use std::env;

use diesel::{
    dsl::insert_into,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    result::{DatabaseErrorKind, Error},
};
use tokio::task;

use crate::{
    domain::game::{
        models::game::{CreateGamesError, NewGame},
        ports::GameService,
    },
    outbound::postgres::dto::NewGameDto,
};

pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(url);

        Self {
            pool: Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .expect("Could not build connection pool"),
        }
    }

    pub async fn save_games(&self, new_games: Vec<NewGame>) -> Result<usize, Error> {
        let pool = self.pool.clone();

        task::spawn_blocking(move || {
            use schema::game::dsl::*;

            let mut conn = pool.get().map_err(|_| Error::BrokenTransactionManager)?;

            let result = insert_into(game)
                .values(
                    new_games
                        .into_iter()
                        .map(NewGameDto::from)
                        .collect::<Vec<NewGameDto>>(),
                )
                .on_conflict_do_nothing()
                .execute(&mut conn)?;

            Ok(result)
        })
        .await
        .map_err(|_| Error::NotInTransaction)? // It will be mapped to CreateGamesError
    }
}

impl GameService for Postgres {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<usize, CreateGamesError> {
        self.save_games(games).await.map_err(|err| match err {
            Error::BrokenTransactionManager => {
                CreateGamesError::ConnectionError("database connection failed".to_string())
            }
            Error::DatabaseError(err_type, _) => CreateGamesError::DatabaseError(
                match err_type {
                    DatabaseErrorKind::CheckViolation => "check violation",
                    DatabaseErrorKind::ForeignKeyViolation => "foreign key violation",
                    DatabaseErrorKind::NotNullViolation => "not null violation",
                    DatabaseErrorKind::UniqueViolation => "unique violation",
                    _ => "unknown",
                }
                .to_string(),
            ),
            err => CreateGamesError::Unknown(anyhow::anyhow!(err)),
        })
    }
}
