use juniper::{FieldError, graphql_subscription, graphql_value};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{Mutex, mpsc::channel};
use tokio_stream::{Stream, wrappers::ReceiverStream};

use crate::{
    domain::{
        game::models::errors::GameRepositoryError,
        platform::models::{PlatformError, PlatformName},
    },
    inbound::graphql::{GraphQLContext, dto::GraphQLPlatformName},
};

#[derive(Clone, Copy, Debug)]
pub struct Subscription;

type FloatStream = Pin<Box<dyn Stream<Item = Result<f64, FieldError>> + Send>>;

/// The root subscription object of the schema
#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn update_user_games(
        #[graphql(context)] ctx: &GraphQLContext,
        username: String,
        platform_name: GraphQLPlatformName,
    ) -> FloatStream {
        let (float_sender, float_receiver) = channel(1000);
        let (progress_sender, mut progress_receiver) = channel(1000);
        let archives_amount: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

        let platform_name: PlatformName = platform_name.into();
        let platform_service = ctx.platform_service.clone();
        let game_service = ctx.game_service.clone();
        let float_sender_clone = float_sender.clone();
        let archives_amount_clone = archives_amount.clone();
        tokio::spawn(async move {
            let latest_game_timestamp_seconds = match game_service
                .get_latest_game_timestamp_seconds(&platform_name, &username)
                .await
            {
                Ok(timestamp) => timestamp,
                Err(e) => {
                    let _ = float_sender_clone.send(Err(e.into())).await;
                    return;
                }
            };

            let (archives_amount, games_receiver) = match platform_service
                .fetch_games(
                    username.clone(),
                    latest_game_timestamp_seconds,
                    platform_name.clone().into(),
                )
                .await
            {
                Ok(receiver) => receiver,
                Err(e) => {
                    let _ = float_sender_clone.send(Err(e.into())).await;
                    return;
                }
            };
            *archives_amount_clone.lock().await = archives_amount;

            match game_service
                .store_games(&platform_name, &username, games_receiver, progress_sender)
                .await
            {
                Err(e) => {
                    let _ = float_sender_clone.send(Err(e.into())).await;
                    return;
                }
                _ => {}
            };
        });

        let float_sender_clone = float_sender.clone();
        let archives_amount_clone = archives_amount.clone();
        tokio::spawn(async move {
            let mut count = 0;
            while let Some(_) = progress_receiver.recv().await {
                count = count + 1;
                let _ = float_sender_clone
                    .send(Ok(
                        (count as f64) / (*archives_amount_clone.lock().await as f64)
                    ))
                    .await;
            }
        });

        Box::pin(ReceiverStream::new(float_receiver))
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
