use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::{
    domain::{game::models::game::NewGame, platform::models::PlatformName},
    outbound::postgres::dto::GameDto,
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
    black: String,
    platform_name: GraphQLPlatformName,
    pgn: String,
}

impl From<GameDto> for GraphQLGame {
    fn from(value: GameDto) -> Self {
        GraphQLGame {
            id: value.id,
            white: value.white,
            black: value.black,
            platform_name: value.platform_name.into(),
            pgn: value.pgn,
        }
    }
}

#[derive(Clone, GraphQLInputObject)]
pub struct GraphQLGameInput {
    white: String,
    black: String,
    platform_name: GraphQLPlatformName,
    pgn: String,
}

impl Into<NewGame> for GraphQLGameInput {
    fn into(self) -> NewGame {
        NewGame::new(self.white, self.black, self.platform_name.into(), self.pgn)
    }
}
