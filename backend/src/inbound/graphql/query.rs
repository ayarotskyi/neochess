use chrono::{Duration, Utc};
use juniper::{FieldResult, graphql_object};

use crate::inbound::graphql::{
    GraphQLContext,
    dto::{GraphQLMoveStat, GraphQLPlatformName},
};

#[derive(Clone, Copy, Debug)]
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn get_move_stats(
        #[graphql(context)] ctx: &GraphQLContext,
        position_fen: String,
        username: String,
        platform_name: GraphQLPlatformName,
        from_timestamp_seconds: Option<i32>,
        to_timestamp_seconds: Option<i32>,
    ) -> FieldResult<Vec<GraphQLMoveStat>> {
        return Ok(Vec::new()); // Placeholder for actual logic to fetch move stats
    }
}
