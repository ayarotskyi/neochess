mod handlers;

use crate::inbound::graphql::{Schema, schema};
use actix_cors::Cors;
use actix_web::{
    App,
    dev::Server,
    http::header,
    middleware,
    web::{self, Data},
};
use anyhow::Context;
use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub addr: SocketAddr,
}

struct AppData {
    pub schema: Schema,
}

pub struct HttpServer {
    server: Server,
}

impl HttpServer {
    pub fn new(config: HttpServerConfig) -> anyhow::Result<Self> {
        Ok(Self {
            server: actix_web::HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(AppData { schema: schema() }))
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allowed_methods(vec!["POST", "GET"])
                            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                            .allowed_header(header::CONTENT_TYPE)
                            .supports_credentials()
                            .max_age(3600),
                    )
                    .wrap(middleware::Compress::default())
                    .wrap(middleware::Logger::default())
                    .service(
                        web::resource("/graphql")
                            .route(web::post().to(handlers::graphql))
                            .route(web::get().to(handlers::graphql)),
                    )
                    .service(
                        web::resource("/playground").route(web::get().to(handlers::playground)),
                    )
            })
            .bind(config.addr)
            .context(format!("failed to listen on {}", config.addr))?
            .run(),
        })
    }
    pub async fn run(self) -> anyhow::Result<()> {
        self.server
            .await
            .context("received error from running server")?;
        Ok(())
    }
}
