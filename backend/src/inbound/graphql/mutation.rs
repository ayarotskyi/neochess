use juniper::{FieldResult, graphql_object};

use crate::inbound::graphql::GraphQLContext;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

/// The root mutation object of the schema
#[graphql_object(context = GraphQLContext)]
impl Mutation {
    async fn hello_world(#[graphql(context)] _ctx: &GraphQLContext) -> FieldResult<String> {
        Ok(format!("Hello, {}!", "world"))
    }
}
