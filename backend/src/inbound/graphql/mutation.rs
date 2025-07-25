use juniper::{FieldResult, graphql_object};
use uuid::Uuid;

use crate::inbound::graphql::{GraphQLContext, dto::GraphQLGameInput};

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

/// The root query object of the schema
#[graphql_object(context = GraphQLContext)]
impl Mutation {
    async fn store_games(
        #[graphql(context)] ctx: &GraphQLContext,
        games: Vec<GraphQLGameInput>,
    ) -> FieldResult<Vec<Uuid>> {
        Ok(ctx
            .game_service
            .store_games(games.into_iter().map(|game| game.into()).collect())
            .await?)
    }
}
