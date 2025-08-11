use crate::domain::{
    game::models::new_game::NewGame,
    platform::models::{PlatformError, PlatformName},
};
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait PlatformApiClient: Send + Sync + 'static {
    async fn fetch_games(
        &self,
        user_name: String,
        from_timestamp_seconds: Option<u64>,
    ) -> Result<(usize, Receiver<Result<Vec<NewGame>, PlatformError>>), PlatformError>;
}

#[async_trait]
pub trait PlatformService: Send + Sync + 'static {
    async fn fetch_games(
        &self,
        user_name: String,
        from_timestamp_seconds: Option<u64>,
        platform_name: PlatformName,
    ) -> Result<(usize, Receiver<Result<Vec<NewGame>, PlatformError>>), PlatformError>;
}
