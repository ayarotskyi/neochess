use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::game::models::game::{CreateGamesError, NewGame};

#[async_trait]
pub trait GameRepository: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, CreateGamesError>;
}

#[async_trait]
pub trait GameService: Send + Sync + 'static {
    async fn store_games(&self, games: Vec<NewGame>) -> Result<Vec<Uuid>, CreateGamesError>;
}
