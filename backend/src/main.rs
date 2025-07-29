pub mod domain;
pub mod inbound;
pub mod outbound;

use crate::{
    domain::{game, platform},
    inbound::http::{HttpServer, HttpServerConfig},
    outbound::postgres::Postgres,
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let postgres = Postgres::new(database_url);
    let game_service = game::service::Service::new(postgres);
    let platform_service = platform::service::Service::new();

    let server = HttpServer::new(server_config, game_service, platform_service).unwrap();

    server.run().await
}
