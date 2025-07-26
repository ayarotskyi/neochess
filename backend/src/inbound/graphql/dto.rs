use std::time::{Duration, UNIX_EPOCH};

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::domain::{
    game::models::game::{Game, NewGame},
    platform::models::PlatformName,
};

#[derive(GraphQLEnum, Clone)]
#[graphql(name = "PlatformName")]
enum GraphQLPlatformName {
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
            platform_name: GraphQLPlatformName::from(*value.platform_name()),
            pgn: value.pgn().to_string(),
            finished_at: value
                .finished_at()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::new(0, 0))
                .as_millis() as i32,
        }
    }
}

#[derive(Clone, GraphQLInputObject)]
pub struct GraphQLGameInput {
    white: String,
    white_elo: i32,
    black: String,
    black_elo: i32,
    platform_name: GraphQLPlatformName,
    pgn: String,
    finished_at: i32,
}

impl Into<NewGame> for GraphQLGameInput {
    fn into(self) -> NewGame {
        NewGame::new(
            self.white,
            self.white_elo as i8,
            self.black,
            self.black_elo as i8,
            self.platform_name.into(),
            self.pgn,
            UNIX_EPOCH
                .checked_add(Duration::from_millis(self.finished_at as u64))
                .unwrap_or(UNIX_EPOCH),
        )
    }
}
