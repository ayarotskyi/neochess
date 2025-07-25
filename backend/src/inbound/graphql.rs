mod dto;
mod mutation;
mod query;

use crate::domain::game::ports::GameService;
use juniper::{Context, EmptySubscription, RootNode};
use mutation::Mutation;
use query::Query;
use std::sync::Arc;

pub struct GraphQLContext {
    game_service: Arc<dyn GameService>,
}

impl GraphQLContext {
    pub fn new(game_service: Arc<dyn GameService>) -> Self {
        Self { game_service }
    }
}

impl Context for GraphQLContext {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
