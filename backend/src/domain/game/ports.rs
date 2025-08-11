use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::domain::{
    game::models::{
        errors::{GameRepositoryError, InvalidFenError, StoreGamesError},
        fen::Fen,
        game::Color,
        move_stat::MoveStat,
        new_game::NewGame,
    },
    platform::models::{PlatformError, PlatformName},
};

#[async_trait]
pub trait GameRepository: Send + Sync + 'static {
    async fn store_games(
        &self,
        platform_name: &PlatformName,
        username: &str,
        game_receiver: Receiver<Result<Vec<NewGame>, PlatformError>>,
        progress_sender: Sender<usize>,
    ) -> Result<(), GameRepositoryError>;

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Option<u64>, GameRepositoryError>;

    async fn get_move_stats(
        &self,
        position_fen: &Fen,
        username: &str,
        play_as: &Color,
        platform_name: &PlatformName,
        from_timestamp: &Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp: &Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError>;
}

#[async_trait]
pub trait GameService: Send + Sync + 'static {
    async fn store_games(
        &self,
        platform_name: &PlatformName,
        username: &str,
        game_receiver: Receiver<Result<Vec<NewGame>, PlatformError>>,
        progress_sender: Sender<usize>,
    ) -> Result<(), StoreGamesError>;

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: &PlatformName,
        username: &str,
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
