use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    game::models::{
        game::{GameRepositoryError, NewGame},
        position::MoveStat,
    },
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

    async fn get_move_stats(
        &self,
        position_fen: String,
        username: String,
        platform_name: PlatformName,
        from_timestamp: Option<i32>,
        to_timestamp: Option<i32>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError>;
}

#[async_trait]
pub trait GameService: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError>;

    async fn get_latest_game_timestamp(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError>;

    async fn get_move_stats(
        &self,
        position_fen: String,
        username: String,
        platform_name: PlatformName,
        from_timestamp: Option<i32>,
        to_timestamp: Option<i32>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError>;
}
