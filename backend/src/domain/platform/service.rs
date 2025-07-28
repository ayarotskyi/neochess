use crate::domain::{
    game::models::game::NewGame,
    platform::{
        models::{PlatformError, PlatformName},
        ports::{FetchGamesParameters, PlatformApiClient, PlatformService},
    },
};

type PlatformApiClientMap = std::collections::HashMap<PlatformName, Box<dyn PlatformApiClient>>;

pub struct Service {
    client_map: PlatformApiClientMap,
}

impl Service {
    pub fn new() -> Self {
        let mut client_map = PlatformApiClientMap::new();
        client_map.insert(
            PlatformName::ChessCom,
            Box::new(crate::outbound::platforms::chesscom::ChessComClient::new()),
        );

        Self { client_map }
    }
}

#[async_trait::async_trait]
impl PlatformService for Service {
    async fn fetch_games(
        &self,
        params: FetchGamesParameters,
        platform_name: PlatformName,
    ) -> Result<Vec<NewGame>, PlatformError> {
        let client = self
            .client_map
            .get(&platform_name)
            .ok_or(PlatformError::PlatformNotFound(
                Into::<&'static str>::into(platform_name).to_string(),
            ))?;

        client.fetch_games(params).await
    }
}
