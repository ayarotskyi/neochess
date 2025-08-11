mod handlers;

use crate::{
    domain::{game::ports::GameService, platform::ports::PlatformService},
    inbound::graphql::{Schema, schema},
};
use actix_cors::Cors;
use actix_web::{
    App,
    dev::Server,
    http::header,
    middleware,
    web::{self, Data},
};
use anyhow::Context;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub addr: SocketAddr,
}

struct AppData<GS: GameService, PS: PlatformService> {
    pub schema: Arc<Schema>,
    pub game_service: Arc<GS>,
    pub platform_service: Arc<PS>,
}

pub struct HttpServer {
    server: Server,
}

impl HttpServer {
    pub fn new<GS: GameService, PS: PlatformService>(
        config: HttpServerConfig,
        game_service: GS,
        platform_service: PS,
    ) -> anyhow::Result<Self> {
        let game_service_arc = Arc::new(game_service);
        let platform_service_arc = Arc::new(platform_service);
        Ok(Self {
            server: actix_web::HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(AppData {
                        schema: Arc::new(schema()),
                        game_service: game_service_arc.clone(),
                        platform_service: platform_service_arc.clone(),
                    }))
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
                        web::resource("/subscriptions")
                            .route(web::get().to(handlers::subscriptions::<GS, PS>)),
                    )
                    .service(
                        web::resource("/graphql")
                            .route(web::post().to(handlers::graphql::<GS, PS>))
                            .route(web::get().to(handlers::graphql::<GS, PS>)),
                    )
                    .service(web::resource("/playground").route(
                        web::get().to(|| handlers::playground("/graphql", "/subscriptions")),
                    ))
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
