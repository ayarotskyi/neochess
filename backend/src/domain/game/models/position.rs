use crate::domain::game::models::fen::Fen;

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

#[cfg(test)]
mod tests {
    use crate::domain::game::models::{errors::InvalidFenError, fen::FenValidator};

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

        let expected = Ok(Fen::new_unchecked(fen_str.into()));

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
