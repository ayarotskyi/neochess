use juniper::graphql_object;

use crate::inbound::graphql::GraphQLContext;

#[derive(Clone, Copy, Debug)]
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = GraphQLContext)]
impl Query {
    fn hello_world(#[graphql(context)] _ctx: &GraphQLContext) -> Option<String> {
        Some(format!("Hello, {}!", "world"))
    }

    async fn get_move_stats(#[graphql(context)] ctx: &GraphQLContext) -> Option<String> {
        None
    }
}
