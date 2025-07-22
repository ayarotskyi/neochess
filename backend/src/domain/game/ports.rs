use crate::domain::game::models::game::{CreateGamesError, NewGame};

pub trait GameService {
    fn store_games(
        &self,
        games: Vec<NewGame>,
    ) -> impl Future<Output = Result<(), CreateGamesError>>;
}
