use chrono::{DateTime, Datelike};
use rayon::prelude::*;
use tokio::task::JoinSet;

use crate::domain::{
    game::models::game::Color,
    game::models::new_game::NewGame,
    platform::{
        models::{PlatformError, PlatformName},
        ports::PlatformApiClient,
    },
};

pub struct ChessComClient {
    client: reqwest::Client,
}

impl ChessComClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        // Set user agent to avoid 403 Forbidden errors
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("neochess/0.1"),
        );
        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    async fn fetch_player_archives(
        &self,
        username: String,
    ) -> Result<ChessComPlayerArchivesResponse, PlatformError> {
        let url = format!(
            "https://api.chess.com/pub/player/{}/games/archives",
            username
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| PlatformError::NetworkError(e.to_string()))?
            .error_for_status()
            .map_err(|e| PlatformError::ApiError(e.to_string()))?;

        response
            .json::<ChessComPlayerArchivesResponse>()
            .await
            .map_err(|e| PlatformError::ParseError(format!("Failed to parse archives: {}", e)))
    }

    async fn fetch_games_by_archives(
        &self,
        archives: Vec<String>,
    ) -> Result<Vec<NewGame>, PlatformError> {
        let mut join_set = JoinSet::new();
        archives.iter().for_each(|archive_url| {
            let client = self.client.clone();
            let url = archive_url.clone();
            join_set.spawn(async move {
                let response = client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| PlatformError::NetworkError(e.to_string()))?
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
            });
        });

        let mut new_games = Vec::new();

        while let Some(result) = join_set.join_next().await {
            new_games.extend(result.map_err(|err| PlatformError::Unknown(err.into()))??);
        }

        Ok(new_games)
    }

    fn filter_archives_by_timestamp(
        &self,
        archives: Vec<String>,
        from_timestamp: u64,
    ) -> Vec<String> {
        match chrono::DateTime::from_timestamp_millis((from_timestamp * 1000) as i64) {
            Some(from_date_time) => archives
                .into_iter()
                .filter(|archive| {
                    let data_arr = archive.split('/').collect::<Vec<&str>>();
                    let month_str = *data_arr.get(data_arr.len() - 1).unwrap_or(&"");
                    let year_str = *data_arr.get(data_arr.len() - 2).unwrap_or(&"");
                    let archive_month = month_str.parse::<u64>().unwrap_or(0);
                    let archive_year = year_str.parse::<u64>().unwrap_or(0);
                    archive_month >= from_date_time.month() as u64
                        && archive_year >= from_date_time.year() as u64
                })
                .collect(),
            None => archives,
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
    pub pgn: Option<String>,
    pub end_time: u64,
    pub white: ChessComPlayerReponse,
    pub black: ChessComPlayerReponse,
}

impl Into<NewGame> for ChessComGameResponse {
    fn into(self) -> NewGame {
        NewGame::new(
            self.white.username,
            self.white.rating as i16,
            self.black.username,
            self.black.rating as i16,
            (self.white.result.to_lowercase() == "win")
                .then_some(Color::White)
                .or_else(|| (self.black.result.to_lowercase() == "win").then_some(Color::Black)),
            PlatformName::ChessCom,
            self.pgn.unwrap_or_default(),
            DateTime::from_timestamp(self.end_time as i64, 0).unwrap_or(DateTime::UNIX_EPOCH),
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
        user_name: String,
        from_timestamp: Option<u64>,
    ) -> Result<Vec<NewGame>, PlatformError> {
        let archives_response = self.fetch_player_archives(user_name.clone()).await?;

        // Filter archives based on the from_timestamp
        let archives = if let Some(timestamp) = from_timestamp {
            self.filter_archives_by_timestamp(archives_response.archives, timestamp)
        } else {
            archives_response.archives
        };

        let new_games = self
            .fetch_games_by_archives(archives)
            .await
            .map(|new_games| {
                new_games
                    .into_par_iter()
                    .filter(|game| {
                        game.finished_at().timestamp() as u64 > from_timestamp.unwrap_or(0)
                    })
                    .collect::<Vec<NewGame>>()
            });

        new_games
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_archives_by_timestamp_filters_correctly() {
        let client = ChessComClient::new();
        // Example archives: year/month at the end
        let archives = vec![
            "https://api.chess.com/pub/player/test/games/2024/03".to_string(),
            "https://api.chess.com/pub/player/test/games/2024/04".to_string(),
            "https://api.chess.com/pub/player/test/games/2024/05".to_string(),
            "https://api.chess.com/pub/player/test/games/2024/06".to_string(),
        ];
        // Timestamp for 2024-05-01
        let from_timestamp = chrono::NaiveDate::from_ymd_opt(2024, 5, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64
            / 1000;

        let filtered = client.filter_archives_by_timestamp(archives, from_timestamp);

        assert!(
            filtered
                .iter()
                .all(|url| url.contains("2024/05") || url.contains("2024/06"))
        );
        assert_eq!(filtered.len(), 2);
    }
}
