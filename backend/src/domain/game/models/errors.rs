use thiserror::Error;

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
    #[error("connection failed: {0}")]
    ConnectionError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
