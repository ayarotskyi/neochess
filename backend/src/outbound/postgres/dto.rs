use std::str::FromStr;

use crate::domain::{
    game::models::{
        fen::Fen,
        game::{Color, Game},
        move_stat::MoveStat,
        new_game::NewGame,
        pgn::Pgn,
        position::Position,
    },
    platform::models::PlatformName,
};

/// DTO for game model
pub struct GameDto {
    pub id: uuid::Uuid,
    pub white: String,
    pub white_elo: i16,
    pub black: String,
    pub black_elo: i16,
    pub winner: Option<String>,
    pub platform_name: String,
    pub pgn: String,
    pub finished_at: chrono::DateTime<chrono::Utc>,
}

impl From<GameDto> for Game {
    fn from(value: GameDto) -> Self {
        Self::new(
            value.id,
            value.white,
            value.white_elo as i16,
            value.black,
            value.black_elo as i16,
            match value.winner {
                Some(value) => Color::from_str(&value).ok(),
                None => None,
            },
            PlatformName::from_str(&value.platform_name).unwrap_or(PlatformName::ChessCom),
            Pgn::new_unchecked(&value.pgn),
            value.finished_at,
        )
    }
}

pub struct NewGameDto {
    pub white: String,
    pub white_elo: i16,
    pub black: String,
    pub black_elo: i16,
    pub winner: Option<String>,
    pub platform_name: String,
    pub pgn: String,
    pub finished_at: chrono::DateTime<chrono::Utc>,
}

impl From<NewGame> for NewGameDto {
    fn from(value: NewGame) -> Self {
        Self {
            white: value.white().clone(),
            white_elo: *value.white_elo() as i16,
            black: value.black().clone(),
            black_elo: *value.black_elo() as i16,
            winner: value
                .winner()
                .map(|color| Into::<&'static str>::into(color.clone()).to_string()),
            platform_name: <&PlatformName as Into<&'static str>>::into(value.platform_name())
                .to_string(),
            pgn: value.pgn().to_string(),
            finished_at: *value.finished_at(),
        }
    }
}

/// DTO for position model
pub struct PositionDto {
    pub id: uuid::Uuid,
    pub fen: String,
}

impl From<PositionDto> for Position {
    fn from(value: PositionDto) -> Self {
        Self::new(value.id, Fen::new_unchecked(&value.fen))
    }
}

pub struct NewPositionDto {
    pub fen: String,
}

pub struct GamePositionDto {
    pub game_id: uuid::Uuid,
    pub position_id: uuid::Uuid,
    pub move_idx: i16,
    pub next_move_uci: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct MoveStatDto {
    pub next_move_uci: String,
    pub total: i64,
    pub wins: i64,
    pub draws: i64,
    pub avg_opponent_elo: i32,
    pub last_played_at: chrono::DateTime<chrono::Utc>,
}

impl Into<MoveStat> for MoveStatDto {
    fn into(self) -> MoveStat {
        MoveStat::new(
            self.next_move_uci,
            self.total as u64,
            self.wins as u64,
            self.draws as u64,
            self.avg_opponent_elo as u16,
            self.last_played_at,
        )
    }
}
