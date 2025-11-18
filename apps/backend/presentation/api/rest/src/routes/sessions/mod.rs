use application::usecase::session::SessionUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::{
    context::JsonErrorStruct, extract::Json, model::ParseableJson as _,
    response::ResponseExt as _,
};
use tap::Pipe as _;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    context::errors::AppError,
    model::{session::JsonVerboseSession, user::LoginJsonUser},
    module::ModulesExt,
};

mod refresh;

pub const SESSIONS_TAG: &str = "sessions";

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
        .routes(routes!(create::<M>))
        .routes(routes!(refresh::refresh::<M>))
}

#[utoipa::path(
    post,
    path = "/",
    tag = SESSIONS_TAG,
    request_body = LoginJsonUser,
    responses(
        (status = CREATED, body = [JsonVerboseSession]),
        (status = BAD_REQUEST, body = JsonErrorStruct),
        (status = NOT_FOUND, body = JsonErrorStruct),
        (status = UNAUTHORIZED, body = JsonErrorStruct),
        (status = INTERNAL_SERVER_ERROR, body = JsonErrorStruct),
    ),
)]
pub async fn create<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<LoginJsonUser>,
) -> Result<impl IntoResponse, AppError> {
    let (username, password) = source.parse()?;
    modules
        .session_usecase()
        .create(username, password)
        .await?
        .pipe(JsonVerboseSession::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}
