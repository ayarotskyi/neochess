use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    game::models::game::{GameRepositoryError, NewGame},
    platform::models::PlatformName,
};

#[async_trait]
pub trait GameRepository: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError>;

    async fn get_latest_game_timestamp(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError>;
}

#[async_trait]
pub trait GameService: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError>;

    async fn get_latest_game_timestamp(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError>;
}
