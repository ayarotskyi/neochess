use diesel::prelude::*;

use crate::{
    domain::{
        game::models::{
            game::{Game, NewGame, Pgn},
            position::{Fen, Position},
        },
        platform::models::PlatformName,
    },
    outbound::postgres::schema::{game, game_position, position},
};

/// DTO for game model
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = game)]
pub struct GameDto {
    pub id: uuid::Uuid,
    pub white: String,
    pub black: String,
    pub platform_name: PlatformName,
    pub pgn: String,
}

impl From<GameDto> for Game {
    fn from(value: GameDto) -> Self {
        Self::new(
            value.id,
            value.white,
            value.black,
            value.platform_name,
            Pgn::new_unchecked(&value.pgn),
        )
    }
}
#[derive(Insertable)]
#[diesel(table_name = game)]
pub struct NewGameDto {
    pub white: String,
    pub black: String,
    pub platform_name: String,
    pub pgn: String,
}

impl From<NewGame> for NewGameDto {
    fn from(value: NewGame) -> Self {
        Self {
            white: value.white().clone(),
            black: value.black().clone(),
            platform_name: <&PlatformName as Into<&'static str>>::into(value.platform_name())
                .to_string(),
            pgn: value.pgn().to_string(),
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
}
