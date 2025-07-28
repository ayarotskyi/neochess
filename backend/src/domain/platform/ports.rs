use crate::domain::{
    game::models::game::NewGame,
    platform::models::{PlatformError, PlatformName},
};
use async_trait::async_trait;

pub struct FetchGamesParameters {
    pub user_name: String,
}

#[async_trait]
pub trait PlatformApiClient: Send + Sync + 'static {
    async fn fetch_games(
        &self,
        params: FetchGamesParameters,
    ) -> Result<Vec<NewGame>, PlatformError>;
}

#[async_trait]
pub trait PlatformService: Send + Sync + 'static {
    async fn fetch_games(
        &self,
        params: FetchGamesParameters,
        platform_name: PlatformName,
    ) -> Result<Vec<NewGame>, PlatformError>;
}
