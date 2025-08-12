use juniper::{FieldError, graphql_subscription, graphql_value};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{Mutex, broadcast, mpsc};
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};

use crate::{
    domain::{
        game::models::errors::GameRepositoryError,
        platform::models::{PlatformError, PlatformName},
    },
    inbound::graphql::{
        GraphQLContext, dto::GraphQLPlatformName, game_update_cache::GameUpdateIdentifier,
    },
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
        let cache_identifier = GameUpdateIdentifier::new(username.clone(), platform_name.clone());
        let mut cache_lock = ctx.game_update_cache.lock().await;
        if let Some(receiver) = cache_lock.get(&cache_identifier) {
            return Box::pin(
                BroadcastStream::new(receiver.resubscribe()).map(|item| match item {
                    Ok(item) => item,
                    Err(e) => Err(anyhow::anyhow!(e).into()),
                }),
            );
        }
        let (float_sender, float_receiver) = broadcast::channel(1000);
        cache_lock.insert(cache_identifier, float_sender.subscribe());

        let (progress_sender, mut progress_receiver) = mpsc::channel(1000);
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
                    let _ = float_sender_clone.send(Err(e.into()));
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
                    let _ = float_sender_clone.send(Err(e.into()));
                    return;
                }
            };
            *archives_amount_clone.lock().await = archives_amount;

            match game_service
                .store_games(&platform_name, &username, games_receiver, progress_sender)
                .await
            {
                Err(e) => {
                    let _ = float_sender_clone.send(Err(e.into()));
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
                let _ = float_sender_clone.send(Ok(
                    (count as f64) / (*archives_amount_clone.lock().await as f64)
                ));
            }
        });

        Box::pin(BroadcastStream::new(float_receiver).map(|item| match item {
            Ok(item) => item,
            Err(e) => Err(anyhow::anyhow!(e).into()),
        }))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserGamesError {
    #[error("Failed to load games from platform")]
    PlatformError(#[from] PlatformError),
    #[error("Internal database error")]
    GameRepositoryError(#[from] GameRepositoryError),
    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

impl juniper::IntoFieldError for UpdateUserGamesError {
    fn into_field_error(self) -> juniper::FieldError {
        juniper::FieldError::new(
            self.to_string(),
            graphql_value!({ "type": "UpdateUserGamesError" }),
        )
    }
}
