use std::fmt::{Display, Formatter};

use crate::domain::game::models::errors::InvalidFenError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A valid Forsyth–Edwards Notation string
pub struct Fen(String);

pub trait FenValidator: Send + Sync + 'static {
    fn is_valid_fen(&self, fen: &str) -> bool;
}

impl Fen {
    pub fn new(fen_str: &str, validator: &impl FenValidator) -> Result<Self, InvalidFenError> {
        if validator.is_valid_fen(fen_str) {
            Ok(Self(fen_str.into()))
        } else {
            Err(InvalidFenError)
        }
    }

    pub fn new_unchecked(fen_str: &str) -> Self {
        Self(fen_str.into())
    }
}

impl Display for Fen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
