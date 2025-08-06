use chrono::{DateTime, Utc};
use strum_macros::{EnumString, IntoStaticStr};

use crate::domain::{game::models::pgn::Pgn, platform::models::PlatformName};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, IntoStaticStr)]
pub enum Color {
    White,
    Black,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    id: uuid::Uuid,
    /// username of the player with white
    white: String,
    white_elo: i16,
    /// username of the player with black
    black: String,
    black_elo: i16,
    winner: Option<Color>,
    /// platform name where the game was played
    platform_name: PlatformName,
    pgn: Pgn,
    finished_at: DateTime<Utc>,
}

impl Game {
    pub fn new(
        id: uuid::Uuid,
        white: String,
        white_elo: i16,
        black: String,
        black_elo: i16,
        winner: Option<Color>,
        platform_name: PlatformName,
        pgn: Pgn,
        finished_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id,
            white: white,
            white_elo: white_elo,
            black: black,
            black_elo: black_elo,
            winner: winner,
            platform_name: platform_name,
            pgn: pgn,
            finished_at: finished_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn white(&self) -> &String {
        &self.white
    }

    pub fn white_elo(&self) -> &i16 {
        &self.white_elo
    }

    pub fn black(&self) -> &String {
        &self.black
    }

    pub fn black_elo(&self) -> &i16 {
        &self.black_elo
    }

    pub fn platform_name(&self) -> &PlatformName {
        &self.platform_name
    }

    pub fn pgn(&self) -> &Pgn {
        &self.pgn
    }

    pub fn finished_at(&self) -> &DateTime<Utc> {
        &self.finished_at
    }

    pub fn winner(&self) -> Option<&Color> {
        self.winner.as_ref()
    }
}
