use std::{str::FromStr, time::SystemTime};

use crate::{
    domain::{
        game::models::{
            game::{Color, Game, NewGame, Pgn},
            position::{Fen, Position},
        },
        platform::models::PlatformName,
    },
    outbound::postgres::schema::{
        game::{self},
        game_position, position,
    },
};
use chrono::{MappedLocalTime, TimeZone, Utc};
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
    finished_at: chrono::DateTime<chrono::Utc>,
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
            finished_at: match Utc.timestamp_millis_opt((*value.finished_at() * 1000) as i64) {
                MappedLocalTime::Single(dt) => dt,
                MappedLocalTime::Ambiguous(dt, _) => dt,
                MappedLocalTime::None => {
                    unreachable!("Invalid timestamp for finished_at: {}", value.finished_at())
                }
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
    pub next_move_san: Option<String>,
}

pub struct MoveStatDto {
    pub next_move_san: String,
    pub total: i64,
    pub wins: i64,
    pub draws: i64,
    pub avg_opponent_elo: i32,
}

impl QueryableByName<Pg> for MoveStatDto {
    fn build<'a>(row: &impl diesel::row::NamedRow<'a, Pg>) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            next_move_san: NamedRow::get::<diesel::sql_types::Text, _>(row, "next_move_san")?,
            total: NamedRow::get(row, "total")?,
            wins: NamedRow::get(row, "wins")?,
            draws: NamedRow::get(row, "draws")?,
            avg_opponent_elo: NamedRow::get(row, "avg_opponent_elo")?,
        })
    }
}
