pub mod domain;
pub mod inbound;
pub mod outbound;

use crate::{
    domain::{
        game,
        platform::{self, models::PlatformName, service::PlatformApiClientMap},
    },
    inbound::http::{HttpServer, HttpServerConfig},
    outbound::{fen_validator, platforms::chesscom::ChessComClient, postgres::Postgres},
};
use std::{env, net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = HttpServerConfig {
        addr: SocketAddr::from_str(&format!(
            "{}:{}",
            std::env::var("HOST").expect("HOST must be set"),
            std::env::var("PORT").expect("PORT must be set")
        ))
        .unwrap(),
    };

    // Prepare the Game Service
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgres = Postgres::new(database_url)?;
    let fen_validator = fen_validator::Validator;
    let game_service = game::service::Service::new(postgres, fen_validator);

    // Prepare the Platform Service
    let platform_api_client_map = construct_platform_api_client_map();
    let platform_service = platform::service::Service::new(platform_api_client_map);

    let server = HttpServer::new(server_config, game_service, platform_service).unwrap();

    server.run().await
}

fn construct_platform_api_client_map() -> PlatformApiClientMap {
    let mut client_map = PlatformApiClientMap::new();

    client_map.insert(PlatformName::ChessCom, Box::new(ChessComClient::new()));

    client_map
}
