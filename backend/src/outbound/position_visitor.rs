use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::{Chess, Position as _};
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

pub struct PositionMovetext {
    pub chess: Chess,
    pub result: Vec<Fen>,
}

impl Visitor for PositionVisitor {
    type Tags = ();
    type Movetext = PositionMovetext;
    type Output = Result<Vec<Fen>, InvalidPgnError>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(())
    }

    fn begin_movetext(&mut self, _tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(PositionMovetext {
            chess: Chess::new(),
            result: Vec::new(),
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
                movetext.result.push(Fen::new_unchecked(
                    &movetext.chess.board().board_fen().to_string(),
                ));
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
