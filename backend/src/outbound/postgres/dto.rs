use std::{
    str::FromStr,
    time::{Duration, SystemTime},
};

use crate::{
    domain::{
        game::models::{
            fen::Fen,
            game::{Color, Game},
            move_stat::MoveStat,
            new_game::NewGame,
            pgn::Pgn,
            position::Position,
        },
        platform::models::PlatformName,
    },
    outbound::postgres::schema::{
        game::{self},
        game_position, position,
    },
};
use chrono::DateTime;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::row::NamedRow;

/// DTO for game model
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = game)]
pub struct GameDto {
    pub id: uuid::Uuid,
    pub white: String,
    pub white_elo: i16,
    pub black: String,
    pub black_elo: i16,
    pub winner: Option<String>,
    pub platform_name: PlatformName,
    pub pgn: String,
    pub finished_at: SystemTime,
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
            value.platform_name,
            Pgn::new_unchecked(&value.pgn),
            value.finished_at,
        )
    }
}
#[derive(Insertable)]
#[diesel(table_name = game)]
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
            finished_at: match DateTime::from_timestamp(
                (match value.finished_at().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(duration) => duration,
                    Err(err) => {
                        unreachable!("New game timestamp is earlier than unix epoch: {}", err)
                    }
                })
                .as_secs() as i64,
                0,
            ) {
                Some(time) => time,
                None => unreachable!(
                    "Invalid timestamp for finished_at: {:?}",
                    value.finished_at()
                ),
            },
        }
    }
}

/// DTO for position model
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = position)]
pub struct PositionDto {
    pub id: uuid::Uuid,
    pub fen: String,
}

impl From<PositionDto> for Position {
    fn from(value: PositionDto) -> Self {
        Self::new(value.id, Fen::new_unchecked(&value.fen))
    }
}

#[derive(Insertable)]
#[diesel(table_name = position)]
pub struct NewPositionDto {
    pub fen: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(primary_key(game_id, position_id, move_idx))]
#[diesel(belongs_to(GameDto, foreign_key = game_id))]
#[diesel(belongs_to(PositionDto, foreign_key = position_id))]
#[diesel(table_name = game_position)]
pub struct GamePositionDto {
    pub game_id: uuid::Uuid,
    pub position_id: uuid::Uuid,
    pub move_idx: i16,
    pub next_move_uci: Option<String>,
}

pub struct MoveStatDto {
    pub next_move_uci: String,
    pub total: i64,
    pub wins: i64,
    pub draws: i64,
    pub avg_opponent_elo: i32,
    pub last_played_at: chrono::DateTime<chrono::Utc>,
}

impl QueryableByName<Pg> for MoveStatDto {
    fn build<'a>(row: &impl diesel::row::NamedRow<'a, Pg>) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            next_move_uci: NamedRow::get::<diesel::sql_types::Text, _>(row, "next_move_uci")?,
            total: NamedRow::get(row, "total")?,
            wins: NamedRow::get(row, "wins")?,
            draws: NamedRow::get(row, "draws")?,
            avg_opponent_elo: NamedRow::get(row, "avg_opponent_elo")?,
            last_played_at: NamedRow::get(row, "last_played_at")?,
        })
    }
}

impl Into<MoveStat> for MoveStatDto {
    fn into(self) -> MoveStat {
        MoveStat::new(
            self.next_move_uci,
            self.total as u64,
            self.wins as u64,
            self.draws as u64,
            self.avg_opponent_elo as u16,
            match SystemTime::UNIX_EPOCH.checked_add(Duration::from_millis(
                self.last_played_at.timestamp_millis() as u64,
            )) {
                Some(time) => time,
                None => unreachable!("Invalid last_played_at value: {}", self.last_played_at),
            },
        )
    }
}
