use application::usecase::user::UserUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::{
    context::JsonErrorStruct, extract::Json, model::ParseableJson as _,
    response::ResponseExt as _,
};
use tap::Pipe as _;

use crate::{
    context::errors::AppError,
    model::user::{JsonUser, RegisterJsonUser},
    module::ModulesExt,
    routes::users::USERS_TAG,
};

#[utoipa::path(
    post,
    path = "/register",
    tag = USERS_TAG,
    request_body = RegisterJsonUser,
    responses(
        (status = CREATED, body = [JsonUser]),
        (status = BAD_REQUEST, body = JsonErrorStruct),
        (status = INTERNAL_SERVER_ERROR, body = JsonErrorStruct),
    ),
)]
pub async fn register<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<RegisterJsonUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = source.parse()?;
    modules
        .user_usecase()
        .register(user)
        .await?
        .pipe(JsonUser::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}
