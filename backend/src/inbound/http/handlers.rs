use crate::inbound::{graphql::GraphQLContext, http::AppData};
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
    #[error("internal error")]
    InternalError,

    #[error("bad request")]
    BadRequest,

    #[error("timeout")]
    Timeout,
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::BadRequest => StatusCode::BAD_REQUEST,
            HttpError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

pub async fn playground() -> Result<HttpResponse, Error> {
    if cfg!(debug_assertions) {
        playground_handler("/graphql", None).await
    } else {
        Err(HttpError::BadRequest.into())
    }
}

pub async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    app_data: Data<AppData>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&app_data.schema, &GraphQLContext::new(), req, payload).await
}
