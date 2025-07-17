pub mod inbound;

use crate::inbound::http::{HttpServer, HttpServerConfig};
use std::{net::SocketAddr, str::FromStr};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let server = HttpServer::new(HttpServerConfig {
        addr: SocketAddr::from_str(&format!(
            "{}:{}",
            std::env::var("IP").expect("IP must be set"),
            std::env::var("PORT").expect("PORT must be set")
        ))
        .unwrap(),
    })
    .unwrap();

    server.run().await
}
