use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    game::ports::{GameRepository, GameService},
    platform::models::PlatformName,
};

#[derive(Debug, Clone, Copy)]
pub struct Service<R>
where
    R: GameRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: GameRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo: repo }
    }
}

#[async_trait]
impl<R> GameService for Service<R>
where
    R: GameRepository,
{
    async fn store_games(
        &self,
        games: Vec<super::models::game::NewGame>,
    ) -> Result<Vec<Uuid>, super::models::game::GameRepositoryError> {
        let result = self.repo.store_games(games).await;

        let _ = result
            .as_ref()
            .inspect_err(|err| eprintln!("failed to store games: {}", *err));

        return result;
    }

    async fn get_latest_game_timestamp_seconds(
        &self,
        platform_name: PlatformName,
        username: String,
    ) -> Result<Option<u64>, super::models::game::GameRepositoryError> {
        let result = self
            .repo
            .get_latest_game_timestamp_seconds(platform_name, username)
            .await;

        let _ = result
            .as_ref()
            .inspect_err(|err| eprintln!("failed to get latest game timestamp: {}", *err));

        return result;
    }

    async fn get_move_stats(
        &self,
        position_fen: String,
        username: String,
        platform_name: PlatformName,
        from_timestamp_seconds: Option<i32>,
        to_timestamp_seconds: Option<i32>,
    ) -> Result<Vec<super::models::position::MoveStat>, super::models::game::GameRepositoryError>
    {
        let result = self
            .repo
            .get_move_stats(
                position_fen,
                username,
                platform_name,
                from_timestamp_seconds,
                to_timestamp_seconds,
            )
            .await;

        let _ = result
            .as_ref()
            .inspect_err(|err| eprintln!("failed to get move stats: {}", *err));

        return result;
    }
}
