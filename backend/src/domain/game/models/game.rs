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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pgn(String);

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("parse pgn error")]
pub struct InvalidPgnError;

impl Pgn {
    pub fn new(pgn_str: &str) -> Result<Self, InvalidPgnError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pgn_success() {
        let pgn_str = "[Event \"Live Chess\"] \n [Site \"Chess.com\"] \n [Date \"2025.07.18\"] \n [Round \"?\"] \n [White \"ayarotskyi\"] \n [Black \"sina6211\"] \n [Result \"1-0\"] \n [TimeControl \"900+10\"] \n [WhiteElo \"1373\"] \n [BlackElo \"1351\"] \n [Termination \"ayarotskyi won by resignation\"] \n [ECO \"C21\"] \n [EndTime \"22:24:46 GMT+0000\"] \n [Link \"https://www.chess.com/game/live/140850165046\"] \n 1. e4 e5 2. d4 exd4 3. c3 dxc3 4. Bc4 cxb2 5. Bxb2 Qe7 6. Qe2 Qb4+ 7. Bc3 Qe7 8. \n Nf3 Nf6 9. e5 Ng8 10. O-O Nc6 11. Nbd2 b6 12. Qe4 Bb7 13. Qf4 O-O-O 14. Bxf7 Nh6 \n 15. Bb3 Qc5 16. Rac1 Be7 17. Rfe1 Rhf8 18. Qg3 Nf5 19. Qg4 Nxe5 20. Bxe5 Bxf3 \n 21. Rxc5 Bxg4 22. Rxc7+ Kb8 23. Rxd7+ Bd6 24. Rxd8+ Rxd8 25. Nc4 Bxe5 26. Nxe5 \n Bh5 27. Nc6+ Kc7 28. Nxd8 Kxd8 29. Re5 Bg6 30. Rd5+ Kc7 31. h3 Ne7 32. Rg5 Nc6 \n 33. Rg3 Nd4 34. Kf1 Nb5 35. Ke1 Nd4 36. Kd2 Nf5 37. Rc3+ Kd8 38. Rc4 Ne7 39. \n Rd4+ Kc7 40. Rg4 Nf5 41. Bc2 Nd6 42. Bxg6 hxg6 43. Rxg6 Nf5 44. g4 Nd4 45. Rxg7+ \n 1-0";

        let expected = Ok(Pgn(pgn_str.into()));

        let actual = Pgn::new(pgn_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_create_pgn_failure() {
        let pgn_str = "...";

        let expected = Err(InvalidPgnError);

        let actual = Pgn::new(pgn_str);

        assert_eq!(expected, actual);
    }
}
