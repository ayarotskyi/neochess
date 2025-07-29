use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::{Chess, Position as _, san::San};
use std::ops::ControlFlow;

use crate::domain::game::models::{game::InvalidPgnError, position::Fen};

pub struct PositionVisitor {
    pgn: String,
}

impl PositionVisitor {
    pub fn new(pgn: String) -> Self {
        Self { pgn: pgn }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PositionMetadata {
    pub fen: Fen,
    pub next_move_san: Option<San>,
}

pub struct PositionMovetext {
    pub chess: Chess,
    pub result: Vec<PositionMetadata>,
}

impl Visitor for PositionVisitor {
    type Tags = ();
    type Movetext = PositionMovetext;
    type Output = Result<Vec<PositionMetadata>, InvalidPgnError>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(())
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(PositionMovetext {
            chess: Chess::new(),
            result: vec![PositionMetadata {
                fen: shakmaty::fen::Fen::default().into(),
                next_move_san: None,
            }],
        })
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        let san = san_plus.san;
        let pos = movetext.chess.clone();
        match san.to_move(&pos) {
            Ok(mv) => {
                movetext.chess = match pos.play(mv) {
                    Ok(chess) => chess,
                    Err(_) => return ControlFlow::Break(Err(InvalidPgnError(self.pgn.clone()))),
                };
                let next_fen = shakmaty::fen::Fen::try_from_setup(
                    movetext.chess.to_setup(shakmaty::EnPassantMode::Always),
                );
                match next_fen {
                    Ok(fen) => {
                        movetext.result.last_mut().unwrap().next_move_san = Some(san);
                        movetext.result.push(PositionMetadata {
                            fen: fen.into(),
                            next_move_san: None,
                        })
                    }
                    Err(_) => return ControlFlow::Break(Err(InvalidPgnError(self.pgn.clone()))),
                };
                ControlFlow::Continue(())
            }
            _ => ControlFlow::Break(Err(InvalidPgnError(self.pgn.clone()))),
        }
    }

    fn begin_variation(
        &mut self,
        _movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true)) // stay in the mainline
    }

    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        Ok(movetext.result)
    }
}

impl From<shakmaty::fen::Fen> for Fen {
    fn from(value: shakmaty::fen::Fen) -> Self {
        Self::new_unchecked(&value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{io, str::FromStr};

    use pgn_reader::Reader;

    use super::*;

    #[test]
    fn test_pgn_success() {
        let pgn = r#"
            [Event "?"]
            [Site "?"]
            [Date "????.??.??"]
            [Round "?"]
            [White "?"]
            [Black "?"]
            [Result "1-0"]
            [Link "https://www.chess.com/terms/scholars-mate-chess"]
                
            1. e4 e5 2. Bc4 Nc6 3. Qh5 g6 {This is a great defensive move by Black.
            It defends the weak f7-pawn, prepares to fianchetto the bishop, starts clearing
            the way for the black king to castle, and also attacks the white queen.} 4. Qf3
            {The queen retreats, but notice how it still aims at the weak f7-pawn.} 4... Bg7
            {If Black is not careful, White can end the game in an instant.} 5. Qxf7# 1-0"#;

        let expected: Result<Vec<PositionMetadata>, InvalidPgnError> = Ok(vec![
            PositionMetadata {
                fen: Fen::new_unchecked("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
                next_move_san: Some(San::from_str("e4").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
                ),
                next_move_san: Some(San::from_str("e5").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2",
                ),
                next_move_san: Some(San::from_str("Bc4").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2",
                ),
                next_move_san: Some(San::from_str("Nc6").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 3",
                ),
                next_move_san: Some(San::from_str("Qh5").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3",
                ),
                next_move_san: Some(San::from_str("g6").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqkbnr/pppp1p1p/2n3p1/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 0 4",
                ),
                next_move_san: Some(San::from_str("Qf3").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 1 4",
                ),
                next_move_san: Some(San::from_str("Bg7").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqk1nr/pppp1pbp/2n3p1/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 2 5",
                ),
                next_move_san: Some(San::from_str("Qxf7#").unwrap()),
            },
            PositionMetadata {
                fen: Fen::new_unchecked(
                    "r1bqk1nr/pppp1Qbp/2n3p1/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 5",
                ),
                next_move_san: None,
            },
        ]);

        let mut reader = Reader::new(io::Cursor::new(pgn));

        let actual = reader
            .read_game(&mut PositionVisitor::new(pgn.into()))
            .unwrap_or(Some(Err(InvalidPgnError(pgn.into()))))
            .unwrap_or(Err(InvalidPgnError(pgn.into())));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pgn_failure() {
        let pgn = r#"
            [Event "?"]
            [Site "?"]
            [Date "????.??.??"]
            [Round "?"]
            [White "?"]
            [Black "?"]
            [Result "1-0"]

            1. e4 e5 2. Bc4 Nc6 3. Qh5 g6 4. Qf3 5. Qxf7# 1-0"#;

        let mut reader = Reader::new(io::Cursor::new(pgn));

        let actual_fens = reader
            .read_game(&mut PositionVisitor::new(pgn.into()))
            .unwrap_or(Some(Err(InvalidPgnError(pgn.into()))))
            .unwrap_or(Err(InvalidPgnError(pgn.into())));

        assert!(actual_fens.is_err());
    }
}
