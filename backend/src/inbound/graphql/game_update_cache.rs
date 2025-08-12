use std::collections::HashMap;

use juniper::FieldError;
use tokio::sync::broadcast::Receiver;

use crate::inbound::graphql::dto::GraphQLPlatformName;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct GameUpdateIdentifier {
    username: String,
    platform_name: GraphQLPlatformName,
}

impl GameUpdateIdentifier {
    pub fn new(username: String, platform_name: GraphQLPlatformName) -> Self {
        Self {
            username,
            platform_name,
        }
    }
}

pub type GameUpdateCache = HashMap<GameUpdateIdentifier, Receiver<Result<f64, FieldError>>>;
