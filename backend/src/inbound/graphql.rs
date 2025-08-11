mod dto;
mod query;
mod subscription;

use crate::{
    domain::{game::ports::GameService, platform::ports::PlatformService},
    inbound::graphql::subscription::Subscription,
};
use juniper::{Context, EmptyMutation, RootNode};
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

pub type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
}
