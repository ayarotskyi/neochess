use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::domain::{
    game::{
        models::{
            errors::{GameRepositoryError, InvalidFenError, StoreGamesError},
            fen::{Fen, FenValidator},
            game::Color,
            move_stat::MoveStat,
            new_game::NewGame,
        },
        ports::{GameRepository, GameService},
    },
    platform::models::{PlatformError, PlatformName},
};

#[derive(Debug, Clone, Copy)]
pub struct Service<R, V>
where
    R: GameRepository,
    V: FenValidator,
{
    repo: R,
    fen_validator: V,
}

impl<R, V> Service<R, V>
where
    R: GameRepository,
    V: FenValidator,
{
    pub fn new(repo: R, fen_validator: V) -> Self {
        Self {
            repo: repo,
            fen_validator: fen_validator,
        }
    }
}

#[async_trait]
impl<R, V> GameService for Service<R, V>
where
    R: GameRepository,
    V: FenValidator,
{
    async fn store_games(
        &self,
        platform_name: &PlatformName,
        username: &str,
        game_receiver: Receiver<Result<Vec<NewGame>, PlatformError>>,
        progress_sender: Sender<usize>,
    ) -> Result<(), StoreGamesError> {
        self.repo
            .store_games(platform_name, username, game_receiver, progress_sender)
            .await
            .inspect_err(|err| eprintln!("failed to store games: {}", *err))
            .map_err(|err| err.into())
    }

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: &PlatformName,
        username: &str,
    ) -> Result<Option<u64>, GameRepositoryError> {
        self.repo
            .get_latest_game_timestamp_seconds(platform_name, username)
            .await
            .inspect_err(|err| eprintln!("failed to get latest game timestamp: {}", *err))
            .map_err(|err| err.into())
    }

    async fn get_move_stats(
        &self,
        position_fen: Fen,
        username: String,
        play_as: Color,
        platform_name: PlatformName,
        from_timestamp_seconds: Option<chrono::DateTime<chrono::Utc>>,
        to_timestamp_seconds: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<MoveStat>, GameRepositoryError> {
        self.repo
            .get_move_stats(
                &position_fen,
                &username,
                &play_as,
                &platform_name,
                &from_timestamp_seconds,
                &to_timestamp_seconds,
            )
            .await
            .inspect_err(|err| eprintln!("failed to get move stats: {}", *err))
            .map_err(|err| err.into())
    }

    fn parse_fen(&self, fen_str: String) -> Result<Fen, InvalidFenError> {
        Fen::new(&fen_str, &self.fen_validator)
    }
}
