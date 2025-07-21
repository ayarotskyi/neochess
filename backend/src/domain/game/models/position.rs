use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    id: uuid::Uuid,
    fen: Fen,
    next_move: Option<Move>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A valid Forsyth–Edwards Notation string
pub struct Fen(String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("parse fen error")]
pub struct InvalidFenError;

impl Fen {
    pub fn new(fen_str: &str) -> Result<Self, InvalidFenError> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A valid algebraic notation string
pub struct Move(String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("invalid move notation")]
pub struct InvalidMoveError;

impl Move {
    pub fn new(move_str: &str) -> Result<Self, InvalidMoveError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fen_success() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let expected = Ok(Fen(fen_str.into()));

        let actual = Fen::new(fen_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_fen_failure() {
        let fen_str = "rnbqkbnr/pp1ppppp/8/2pB1R b KQkq - 1 2";

        let expected = Err(InvalidFenError);

        let actual = Fen::new(fen_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_move_success() {
        let move_str = "Qa1xe4+";

        let expected = Ok(Move(move_str.into()));

        let actual = Move::new(move_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_move_failure() {
        let move_str = "Bld0j9Z4";

        let expected = Err(InvalidMoveError);

        let actual = Move::new(move_str);

        assert_eq!(expected, actual);
    }
}
