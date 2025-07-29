mod dto;
mod mutation;
mod query;

use crate::domain::{game::ports::GameService, platform::ports::PlatformService};
use juniper::{Context, EmptySubscription, RootNode};
use mutation::Mutation;
use query::Query;
use std::sync::Arc;

pub struct GraphQLContext {
    game_service: Arc<dyn GameService>,
    platform_service: Arc<dyn PlatformService>,
}

impl GraphQLContext {
    pub fn new(
        game_service: Arc<dyn GameService>,
        platform_service: Arc<dyn PlatformService>,
    ) -> Self {
        Self {
            game_service,
            platform_service,
        }
    }
}

impl Context for GraphQLContext {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
