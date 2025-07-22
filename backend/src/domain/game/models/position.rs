use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    id: uuid::Uuid,
    fen: Fen,
}

impl Position {
    pub fn new(id: uuid::Uuid, fen: Fen) -> Self {
        Self { id: id, fen: fen }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A valid Forsyth–Edwards Notation string
pub struct Fen(String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("parse fen error")]
pub struct InvalidFenError;

pub trait FenValidator {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct ShakmatyFenValidator;

    impl FenValidator for ShakmatyFenValidator {
        fn is_valid_fen(&self, fen: &str) -> bool {
            shakmaty::fen::Fen::from_ascii(fen.as_bytes()).is_ok()
        }
    }

    #[test]
    fn test_create_fen_success() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let expected = Ok(Fen(fen_str.into()));

        let actual = Fen::new(fen_str, &ShakmatyFenValidator);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_fen_failure() {
        let fen_str = "rnbqkbnr/pp1ppppp/8/2pB1R b KQkq - 1 2";

        let expected = Err(InvalidFenError);

        let actual = Fen::new(fen_str, &ShakmatyFenValidator);

        assert_eq!(expected, actual);
    }
}
