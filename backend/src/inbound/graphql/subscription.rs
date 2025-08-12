use juniper::{FieldError, graphql_subscription, graphql_value};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{Mutex, broadcast, mpsc};
use tokio_stream::{
    Stream, StreamExt,
    wrappers::{BroadcastStream, errors::BroadcastStreamRecvError},
};

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

type ProgressStream = Pin<Box<dyn Stream<Item = Result<f64, FieldError>> + Send>>;

/// The root subscription object of the schema.
#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn update_user_games(
        #[graphql(context)] ctx: &GraphQLContext,
        username: String,
        platform_name: GraphQLPlatformName,
    ) -> ProgressStream {
        // Unique key for caching in-progress subscriptions
        let request_key = GameUpdateIdentifier::new(username.clone(), platform_name.clone());

        // Helper to map broadcast errors into GraphQL FieldError
        let map_broadcast_item = |item: Result<_, BroadcastStreamRecvError>| match item {
            Ok(value) => value,
            Err(err) => Err(UpdateUserGamesError::Unknown(anyhow::anyhow!(err)).into()),
        };

        // Check if there's already an identical subscription in progress
        let mut cache = ctx.game_update_cache.lock().await;
        if let Some(existing_rx) = cache.get(&request_key) {
            return Box::pin(
                BroadcastStream::new(existing_rx.resubscribe()).map(map_broadcast_item),
            );
        }

        // Create a new broadcast channel for this subscription
        let (progress_tx, progress_rx) = broadcast::channel::<Result<f64, FieldError>>(1000);
        cache.insert(request_key, progress_tx.subscribe());

        // Channel for reporting discrete progress steps (game count increments)
        let (step_tx, mut step_rx) = mpsc::channel(1000);
        let total_archives = Arc::new(Mutex::new(0usize));

        // Shared service handles
        let platform_name_internal: PlatformName = platform_name.into();
        let platform_service = ctx.platform_service.clone();
        let game_service = ctx.game_service.clone();

        // Spawn the background job to fetch & store games
        {
            let progress_tx = progress_tx.clone();
            let step_tx = step_tx.clone();
            let total_archives = total_archives.clone();
            let username = username.clone();
            let platform_name = platform_name_internal.clone();

            tokio::spawn(async move {
                // Step 1: Find the most recent stored game timestamp
                let latest_timestamp = match game_service
                    .get_latest_game_timestamp_seconds(&platform_name, &username)
                    .await
                {
                    Ok(ts) => ts,
                    Err(err) => {
                        let _ = progress_tx.send(Err(err.into()));
                        return;
                    }
                };

                // Step 2: Fetch games from the platform
                let (archive_count, game_stream) = match platform_service
                    .fetch_games(
                        username.clone(),
                        latest_timestamp,
                        platform_name.clone().into(),
                    )
                    .await
                {
                    Ok(result) => result,
                    Err(err) => {
                        let _ = progress_tx.send(Err(err.into()));
                        return;
                    }
                };
                *total_archives.lock().await = archive_count;

                // Step 3: Store games while reporting progress
                if let Err(err) = game_service
                    .store_games(&platform_name, &username, game_stream, step_tx)
                    .await
                {
                    let _ = progress_tx.send(Err(err.into()));
                }
            });
        }

        // Spawn a progress tracker to convert game count to fraction completed
        {
            let progress_tx = progress_tx.clone();
            let total_archives = total_archives.clone();

            tokio::spawn(async move {
                let mut processed_count = 0usize;

                while let Some(_) = step_rx.recv().await {
                    processed_count += 1;

                    let fraction =
                        processed_count as f64 / (*total_archives.lock().await as f64).max(1.0);

                    let _ = progress_tx.send(Ok(fraction));
                }
            });
        }

        // Return the broadcast stream mapped to the correct GraphQL type
        Box::pin(BroadcastStream::new(progress_rx).map(map_broadcast_item))
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
