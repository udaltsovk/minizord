use application::usecase::session::SessionUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::{
    context::JsonErrorStruct, extract::Json, response::ResponseExt as _,
};
use tap::Pipe as _;

use crate::{
    context::errors::AppError, model::session::JsonSessionTokenPair,
    module::ModulesExt, routes::sessions::SESSIONS_TAG,
};

#[utoipa::path(
    post,
    path = "/refresh",
    tag = SESSIONS_TAG,
    responses(
        (status = OK, body = [JsonSessionTokenPair]),
        (status = BAD_REQUEST, body = JsonErrorStruct),
        (status = NOT_FOUND, body = JsonErrorStruct),
        (status = UNAUTHORIZED, body = JsonErrorStruct),
        (status = INTERNAL_SERVER_ERROR, body = JsonErrorStruct),
    ),
)]
pub async fn refresh<M: ModulesExt>(
    modules: State<M>,
) -> Result<impl IntoResponse, AppError> {
    // let token = source.parse()?;
    modules
        .session_usecase()
        .refresh("".to_string().into())
        .await?
        .pipe(JsonSessionTokenPair::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}
