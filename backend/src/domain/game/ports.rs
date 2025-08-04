use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    game::models::{
        errors::{GameRepositoryError, InvalidFenError},
        fen::Fen,
        game::Color,
        move_stat::MoveStat,
        new_game::NewGame,
    },
    platform::models::PlatformName,
};

#[async_trait]
pub trait GameRepository: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError>;

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError>;

    async fn get_move_stats(
        &self,
        position_fen: Fen,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError>;
}

#[async_trait]
pub trait GameService: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, GameRepositoryError>;

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, GameRepositoryError>;

    async fn get_move_stats(
        &self,
        position_fen: Fen,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError>;

    fn parse_fen(&self, fen_str: String) -> Result<Fen, InvalidFenError>;
}
