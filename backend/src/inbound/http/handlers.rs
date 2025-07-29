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
use juniper_actix::{graphql_handler, playground_handler};
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

pub async fn playground(graphql_endpoint_url: &str) -> Result<HttpResponse, Error> {
    if cfg!(debug_assertions) {
        playground_handler(graphql_endpoint_url, None).await
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
        ),
        req,
        payload,
    )
    .await
}
