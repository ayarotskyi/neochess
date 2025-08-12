mod dto;
pub mod game_update_cache;
mod query;
mod subscription;

use crate::{
    domain::{game::ports::GameService, platform::ports::PlatformService},
    inbound::graphql::{game_update_cache::GameUpdateCache, subscription::Subscription},
};
use juniper::{Context, EmptyMutation, RootNode};
use query::Query;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GraphQLContext {
    game_service: Arc<dyn GameService>,
    platform_service: Arc<dyn PlatformService>,
    game_update_cache: Arc<Mutex<GameUpdateCache>>,
}

impl GraphQLContext {
    pub fn new(
        game_service: Arc<dyn GameService>,
        platform_service: Arc<dyn PlatformService>,
        game_update_cache: Arc<Mutex<GameUpdateCache>>,
    ) -> Self {
        Self {
            game_service,
            platform_service,
            game_update_cache,
        }
    }
}

impl Context for GraphQLContext {}

pub type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
}
