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
    fn hello_world(#[graphql(context)] _ctx: &GraphQLContext) -> Option<String> {
        Some(format!("Hello, {}!", "world"))
    }

    #[graphql(ignore)]
    async fn update_user_games(
        &self,
        ctx: &GraphQLContext,
        username: String,
        platform_name: GraphQLPlatformName,
    ) -> anyhow::Result<()> {
        let latest_game_timestamp = ctx
            .game_service
            .get_latest_game_timestamp(platform_name.clone().into(), username.clone())
            .await?;

        match latest_game_timestamp {
            Some(timestamp) => {
                // if the latest game was one day ago or more, refetch games
                if Duration::milliseconds(timestamp as i64 - Utc::now().timestamp_millis())
                    > Duration::days(1)
                {
                    ctx.game_service
                        .store_games(
                            ctx.platform_service
                                .fetch_games(username, Some(timestamp), platform_name.into())
                                .await?,
                        )
                        .await?;
                }
            }
            // if user has no games, fetch from the platform
            _ => {
                ctx.game_service
                    .store_games(
                        ctx.platform_service
                            .fetch_games(username, None, platform_name.into())
                            .await?,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn get_move_stats(
        &self,
        #[graphql(context)] ctx: &GraphQLContext,
        position_fen: String,
        username: String,
        platform_name: GraphQLPlatformName,
        from_timestamp: Option<i32>,
        to_timestamp: Option<i32>,
    ) -> FieldResult<Vec<GraphQLMoveStat>> {
        self.update_user_games(ctx, username.clone(), platform_name.clone())
            .await?;

        return Ok(Vec::new()); // Placeholder for actual logic to fetch move stats
    }
}
