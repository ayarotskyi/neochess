use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::game::ports::{GameRepository, GameService};

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
    ) -> Result<Vec<Uuid>, super::models::game::CreateGamesError> {
        let result = self.repo.store_games(games).await;

        let _ = result
            .as_ref()
            .inspect_err(|err| eprintln!("failed to store games: {}", *err));

        return result;
    }
}
