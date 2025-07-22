use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::domain::platform::models::PlatformName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    id: uuid::Uuid,
    /// identifier of the player with white
    white: String,
    /// identifier of the player with black
    black: String,
    platform_name: PlatformName,
    pgn: Pgn,
}

impl Game {
    pub fn new(
        id: uuid::Uuid,
        white: String,
        black: String,
        platform_name: PlatformName,
        pgn: Pgn,
    ) -> Self {
        Self {
            id: id,
            white: white,
            black: black,
            platform_name: platform_name,
            pgn: pgn,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn white(&self) -> &String {
        &self.white
    }

    pub fn black(&self) -> &String {
        &self.black
    }

    pub fn platform_name(&self) -> &PlatformName {
        &self.platform_name
    }

    pub fn pgn(&self) -> &Pgn {
        &self.pgn
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewGame {
    white: String,
    black: String,
    platform_name: PlatformName,
    pgn: String,
}

impl NewGame {
    pub fn new(
        white: String,
        black: String,
        platform_name: PlatformName,
        pgn: String,
    ) -> Result<Self, InvalidPgnError> {
        Ok(Self {
            white: white,
            black: black,
            platform_name: platform_name,
            pgn: pgn,
        })
    }

    pub fn white(&self) -> &String {
        &self.white
    }

    pub fn black(&self) -> &String {
        &self.black
    }

    pub fn platform_name(&self) -> &PlatformName {
        &self.platform_name
    }

    pub fn pgn(&self) -> &String {
        &self.pgn
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A valid PGN string
pub struct Pgn(String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("invalid pgn: {0}")]
pub struct InvalidPgnError(pub String);

pub trait PgnValidator {
    fn is_valid_pgn(&self, pgn: &str) -> bool;
}

impl Pgn {
    pub fn new(pgn_str: &str, validator: &impl PgnValidator) -> Result<Self, InvalidPgnError> {
        if validator.is_valid_pgn(pgn_str) {
            Ok(Self(pgn_str.into()))
        } else {
            Err(InvalidPgnError(pgn_str.into()))
        }
    }

    /// Used only when png was taken from trusted sources, like db or game platform
    pub fn new_unchecked(pgn_str: &str) -> Self {
        Self(pgn_str.into())
    }
}

impl Display for Pgn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Error)]
pub enum CreateGamesError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("connection failed: {0}")]
    ConnectionError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use std::{io, ops::ControlFlow};

    use super::*;

    struct MoveCounter;

    impl pgn_reader::Visitor for MoveCounter {
        type Tags = ();
        type Movetext = usize;
        type Output = usize;

        fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
            ControlFlow::Continue(())
        }

        fn begin_movetext(
            &mut self,
            _tags: Self::Tags,
        ) -> ControlFlow<Self::Output, Self::Movetext> {
            ControlFlow::Continue(0)
        }

        fn san(
            &mut self,
            movetext: &mut Self::Movetext,
            _san_plus: pgn_reader::SanPlus,
        ) -> ControlFlow<Self::Output> {
            *movetext += 1;
            ControlFlow::Continue(())
        }

        fn begin_variation(
            &mut self,
            _movetext: &mut Self::Movetext,
        ) -> ControlFlow<Self::Output, pgn_reader::Skip> {
            ControlFlow::Continue(pgn_reader::Skip(true)) // stay in the mainline
        }

        fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
            movetext
        }
    }

    struct PgnReaderPgnValidator;

    impl PgnValidator for PgnReaderPgnValidator {
        fn is_valid_pgn(&self, pgn: &str) -> bool {
            let mut reader = pgn_reader::Reader::new(io::Cursor::new(pgn));
            let result = reader.read_game(&mut MoveCounter);
            result.is_ok_and(|value| value.is_some_and(|value| value > 0))
        }
    }

    #[test]
    fn test_create_pgn_success() {
        let pgn_str = "[Event \"Live Chess\"] \n [Site \"Chess.com\"] \n [Date \"2025.07.18\"] \n [Round \"?\"] \n [White \"ayarotskyi\"] \n [Black \"sina6211\"] \n [Result \"1-0\"] \n [TimeControl \"900+10\"] \n [WhiteElo \"1373\"] \n [BlackElo \"1351\"] \n [Termination \"ayarotskyi won by resignation\"] \n [ECO \"C21\"] \n [EndTime \"22:24:46 GMT+0000\"] \n [Link \"https://www.chess.com/game/live/140850165046\"] \n 1. e4 e5 2. d4 exd4 3. c3 dxc3 4. Bc4 cxb2 5. Bxb2 Qe7 6. Qe2 Qb4+ 7. Bc3 Qe7 8. \n Nf3 Nf6 9. e5 Ng8 10. O-O Nc6 11. Nbd2 b6 12. Qe4 Bb7 13. Qf4 O-O-O 14. Bxf7 Nh6 \n 15. Bb3 Qc5 16. Rac1 Be7 17. Rfe1 Rhf8 18. Qg3 Nf5 19. Qg4 Nxe5 20. Bxe5 Bxf3 \n 21. Rxc5 Bxg4 22. Rxc7+ Kb8 23. Rxd7+ Bd6 24. Rxd8+ Rxd8 25. Nc4 Bxe5 26. Nxe5 \n Bh5 27. Nc6+ Kc7 28. Nxd8 Kxd8 29. Re5 Bg6 30. Rd5+ Kc7 31. h3 Ne7 32. Rg5 Nc6 \n 33. Rg3 Nd4 34. Kf1 Nb5 35. Ke1 Nd4 36. Kd2 Nf5 37. Rc3+ Kd8 38. Rc4 Ne7 39. \n Rd4+ Kc7 40. Rg4 Nf5 41. Bc2 Nd6 42. Bxg6 hxg6 43. Rxg6 Nf5 44. g4 Nd4 45. Rxg7+ \n 1-0";

        let expected = Ok(Pgn(pgn_str.into()));

        let actual = Pgn::new(pgn_str, &PgnReaderPgnValidator);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_pgn_failure() {
        let pgn_str = "...";

        let expected = Err(InvalidPgnError(pgn_str.into()));

        let actual = Pgn::new(pgn_str, &PgnReaderPgnValidator);

        assert_eq!(expected, actual);
    }
}
