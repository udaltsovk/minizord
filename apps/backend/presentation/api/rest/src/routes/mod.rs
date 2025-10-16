use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::response::ResponseExt as _;
use tap::Pipe as _;
use utoipa::OpenApi as _;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    context::{errors::AppError, openapi::ApiDoc},
    model::info::ApiInfo,
    module::ModulesExt,
};

pub mod sessions;
pub mod users;

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(info::<M>))
        .nest("/users", users::router())
        .nest("/sessions", sessions::router())
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = ApiInfo),
    )
)]
pub async fn info<M: ModulesExt>(
    modules: State<M>,
) -> Result<impl IntoResponse, AppError> {
    ApiInfo::get(modules.base_api_url())
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
