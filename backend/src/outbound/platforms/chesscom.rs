use anyhow::anyhow;

use crate::domain::{
    game::models::game::{Color, NewGame},
    platform::{
        models::{PlatformError, PlatformName},
        ports::{FetchGamesParameters, PlatformApiClient},
    },
};

pub struct ChessComClient {
    client: reqwest::Client,
}

impl ChessComClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[derive(serde::Deserialize)]
struct ChessComPlayerArchivesResponse {
    pub archives: Vec<String>,
}

#[derive(serde::Deserialize)]
struct ChessComPlayerReponse {
    pub username: String,
    pub rating: u32,
    pub result: String,
}

#[derive(serde::Deserialize)]
struct ChessComGameResponse {
    pub pgn: String,
    pub end_time: u64,
    pub white: ChessComPlayerReponse,
    pub black: ChessComPlayerReponse,
}

impl Into<NewGame> for ChessComGameResponse {
    fn into(self) -> NewGame {
        NewGame::new(
            self.white.username,
            self.white.rating as i8,
            self.black.username,
            self.black.rating as i8,
            (self.white.result.to_lowercase() == "win")
                .then_some(Color::White)
                .or_else(|| (self.black.result.to_lowercase() == "win").then_some(Color::Black)),
            PlatformName::ChessCom,
            self.pgn,
            self.end_time,
        )
    }
}

#[derive(serde::Deserialize)]
struct ChessComArchiveResponse {
    pub games: Vec<ChessComGameResponse>,
}

#[async_trait::async_trait]
impl PlatformApiClient for ChessComClient {
    async fn fetch_games(
        &self,
        params: FetchGamesParameters,
    ) -> Result<Vec<NewGame>, PlatformError> {
        let url = format!(
            "https://api.chess.com/pub/player/{}/games/archives",
            params.user_name
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| PlatformError::NetworkError(anyhow!(e)))?
            .error_for_status()
            .map_err(|e| PlatformError::ApiError(e.to_string()))?;

        let archives: Vec<String> = response
            .json::<ChessComPlayerArchivesResponse>()
            .await
            .map_err(|e| PlatformError::ParseError(format!("Failed to parse archives: {}", e)))?
            .archives;

        let handles = archives
            .iter()
            .map(|archive_url| {
                let client = self.client.clone();
                let url = archive_url.clone();
                tokio::spawn(async move {
                    let response = client
                        .get(&url)
                        .send()
                        .await
                        .map_err(|e| PlatformError::NetworkError(anyhow!(e)))?
                        .error_for_status()
                        .map_err(|e| PlatformError::ApiError(e.to_string()))?;

                    let archive: ChessComArchiveResponse = response.json().await.map_err(|e| {
                        PlatformError::ParseError(format!("Failed to parse games: {}", e))
                    })?;

                    let games = archive
                        .games
                        .into_iter()
                        .map(|game| game.into())
                        .collect::<Vec<NewGame>>();

                    Ok::<Vec<NewGame>, PlatformError>(games)
                })
            })
            .collect::<Vec<_>>();

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => {
                    results.extend(result?);
                }
                Err(e) => return Err(PlatformError::NetworkError(anyhow!(e))),
            }
        }

        Ok(results)
    }
}
