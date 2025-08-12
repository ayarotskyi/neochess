use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::domain::{
    game::models::{
        game::{Color, Game},
        move_stat::MoveStat,
    },
    platform::models::PlatformName,
};

#[derive(GraphQLEnum, Clone, PartialEq, Eq, Hash, Debug)]
#[graphql(name = "PlatformName")]
pub enum GraphQLPlatformName {
    ChessCom,
}

impl Into<PlatformName> for GraphQLPlatformName {
    fn into(self) -> PlatformName {
        match self {
            GraphQLPlatformName::ChessCom => PlatformName::ChessCom,
        }
    }
}

impl From<PlatformName> for GraphQLPlatformName {
    fn from(value: PlatformName) -> Self {
        match value {
            PlatformName::ChessCom => GraphQLPlatformName::ChessCom,
        }
    }
}

#[derive(Clone, GraphQLObject)]
pub struct GraphQLGame {
    id: Uuid,
    white: String,
    white_elo: i32,
    black: String,
    black_elo: i32,
    winner: Option<GraphQLColor>,
    platform_name: GraphQLPlatformName,
    pgn: String,
    finished_at: i32,
}

impl From<Game> for GraphQLGame {
    fn from(value: Game) -> Self {
        GraphQLGame {
            id: *value.id(),
            white: value.white().clone(),
            white_elo: *value.white_elo() as i32,
            black: value.black().clone(),
            black_elo: *value.black_elo() as i32,
            winner: value.winner().map(|c| GraphQLColor::from(*c)),
            platform_name: GraphQLPlatformName::from(*value.platform_name()),
            pgn: value.pgn().to_string(),
            finished_at: value.finished_at().timestamp() as i32,
        }
    }
}

#[derive(GraphQLEnum, Clone)]
#[graphql(name = "Color")]
pub enum GraphQLColor {
    White,
    Black,
}

impl Into<Color> for GraphQLColor {
    fn into(self) -> Color {
        match self {
            GraphQLColor::White => Color::White,
            GraphQLColor::Black => Color::Black,
        }
    }
}

impl From<Color> for GraphQLColor {
    fn from(value: Color) -> Self {
        match value {
            Color::White => GraphQLColor::White,
            Color::Black => GraphQLColor::Black,
        }
    }
}

#[derive(GraphQLObject, Clone)]
pub struct GraphQLMoveStat {
    pub move_uci: String,
    pub total: i32,
    pub wins: i32,
    pub draws: i32,
    pub avg_opponent_elo: i32,
    pub last_played_at: i32,
}

impl From<MoveStat> for GraphQLMoveStat {
    fn from(value: MoveStat) -> Self {
        GraphQLMoveStat {
            move_uci: value.move_uci().to_string(),
            total: *value.total() as i32,
            wins: *value.wins() as i32,
            draws: *value.draws() as i32,
            avg_opponent_elo: *value.avg_opponent_elo() as i32,
            last_played_at: value.last_played_at().timestamp() as i32,
        }
    }
}
