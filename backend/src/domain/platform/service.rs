use crate::domain::{
    game::models::new_game::NewGame,
    platform::{
        models::{PlatformError, PlatformName},
        ports::{PlatformApiClient, PlatformService},
    },
};

pub type PlatformApiClientMap = std::collections::HashMap<PlatformName, Box<dyn PlatformApiClient>>;

pub struct Service {
    client_map: PlatformApiClientMap,
}

impl Service {
    pub fn new(client_map: PlatformApiClientMap) -> Self {
        Self { client_map }
    }
}

#[async_trait::async_trait]
impl PlatformService for Service {
    async fn fetch_games(
        &self,
        user_name: String,
        from_timestamp_seconds: Option<u64>,
        platform_name: PlatformName,
    ) -> Result<Vec<NewGame>, PlatformError> {
        let client = self
            .client_map
            .get(&platform_name)
            .ok_or(PlatformError::PlatformNotFound(
                Into::<&'static str>::into(platform_name).to_string(),
            ))?;

        client.fetch_games(user_name, from_timestamp_seconds).await
    }
}
