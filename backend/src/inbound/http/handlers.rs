use std::time::Duration;

use crate::{
    domain::{game::ports::GameService, platform::ports::PlatformService},
    inbound::{graphql::GraphQLContext, http::AppData},
};
use actix_http::StatusCode;
use actix_web::{
    Error, HttpRequest, HttpResponse, ResponseError,
    http::header::ContentType,
    web::{self, Data},
};
use juniper_actix::{graphql_handler, playground_handler, subscriptions};
use juniper_graphql_ws::ConnectionConfig;
use thiserror::Error;

#[derive(Error, Debug)]
enum HttpError {
    #[error("bad request")]
    BadRequest,
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn playground(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: &'static str,
) -> Result<HttpResponse, Error> {
    if cfg!(debug_assertions) {
        playground_handler(graphql_endpoint_url, Some(subscriptions_endpoint_url)).await
    } else {
        Err(HttpError::BadRequest.into())
    }
}

pub async fn graphql<GS: GameService, PS: PlatformService>(
    req: HttpRequest,
    payload: web::Payload,
    app_data: Data<AppData<GS, PS>>,
) -> Result<HttpResponse, Error>
where
    GS: GameService,
{
    graphql_handler(
        &app_data.schema,
        &GraphQLContext::new(
            app_data.game_service.clone(),
            app_data.platform_service.clone(),
            app_data.game_update_cache.clone(),
        ),
        req,
        payload,
    )
    .await
}

pub async fn subscriptions<GS: GameService, PS: PlatformService>(
    req: HttpRequest,
    stream: web::Payload,
    app_data: Data<AppData<GS, PS>>,
) -> Result<HttpResponse, Error>
where
    GS: GameService,
{
    let context = GraphQLContext::new(
        app_data.game_service.clone(),
        app_data.platform_service.clone(),
        app_data.game_update_cache.clone(),
    );

    let schema = app_data.schema.clone();
    let config = ConnectionConfig::new(context);
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions::ws_handler(req, stream, schema, config).await
}
