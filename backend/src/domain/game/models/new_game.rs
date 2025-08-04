use std::time::SystemTime;

use crate::domain::{game::models::game::Color, platform::models::PlatformName};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewGame {
    white: String,
    white_elo: i16,
    black: String,
    black_elo: i16,
    winner: Option<Color>,
    platform_name: PlatformName,
    pgn: String,
    finished_at: SystemTime,
}

impl NewGame {
    pub fn new(
        white: String,
        white_elo: i16,
        black: String,
        black_elo: i16,
        winner: Option<Color>,
        platform_name: PlatformName,
        pgn: String,
        finished_at: SystemTime,
    ) -> Self {
        Self {
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

    pub fn winner(&self) -> Option<&Color> {
        self.winner.as_ref()
    }

    pub fn platform_name(&self) -> &PlatformName {
        &self.platform_name
    }

    pub fn pgn(&self) -> &String {
        &self.pgn
    }

    pub fn finished_at(&self) -> &SystemTime {
        &self.finished_at
    }
}
