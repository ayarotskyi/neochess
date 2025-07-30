use juniper::{FieldResult, graphql_object, graphql_value};

use crate::{
    domain::{game::models::game::GameRepositoryError, platform::models::PlatformError},
    inbound::graphql::{GraphQLContext, dto::GraphQLPlatformName},
};

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

/// The root mutation object of the schema
#[graphql_object(context = GraphQLContext)]
impl Mutation {
    async fn update_user_games(
        #[graphql(context)] ctx: &GraphQLContext,
        username: String,
        platform_name: GraphQLPlatformName,
    ) -> FieldResult<i32> {
        let latest_game_timestamp_seconds = ctx
            .game_service
            .get_latest_game_timestamp_seconds(platform_name.clone().into(), username.clone())
            .await?;

        let result = ctx
            .game_service
            .store_games(
                ctx.platform_service
                    .fetch_games(
                        username,
                        latest_game_timestamp_seconds,
                        platform_name.into(),
                    )
                    .await?,
            )
            .await?
            .len();

        Ok(result as i32)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserGamesError {
    #[error("Failed to update user games: {0}")]
    UpdateError(#[from] anyhow::Error),
}

impl From<GameRepositoryError> for UpdateUserGamesError {
    fn from(err: GameRepositoryError) -> Self {
        UpdateUserGamesError::UpdateError(anyhow::anyhow!(err))
    }
}

impl From<PlatformError> for UpdateUserGamesError {
    fn from(err: PlatformError) -> Self {
        UpdateUserGamesError::UpdateError(anyhow::anyhow!(err))
    }
}

impl juniper::IntoFieldError for UpdateUserGamesError {
    fn into_field_error(self) -> juniper::FieldError {
        juniper::FieldError::new(
            self.to_string(),
            graphql_value!({ "type": "UpdateUserGamesError" }),
        )
    }
}
