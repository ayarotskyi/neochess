use thiserror::Error;

use crate::domain::platform::models::PlatformError;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("invalid pgn: {0}")]
pub struct InvalidPgnError(pub String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("parse fen error")]
pub struct InvalidFenError;

#[derive(Debug, Error)]
pub enum GameRepositoryError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
#[derive(Debug, Error)]
pub enum StoreGamesError {
    #[error(transparent)]
    GameRepositoryError(#[from] GameRepositoryError),
    #[error(transparent)]
    PlatformError(#[from] PlatformError),
}
