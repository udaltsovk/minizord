use actix_web::{
    get,
    web::{Data, Json},
};
use macros::implementation;

use super::{ApiInfoResponse, InfoHandler, InfoHandlerResult};
use crate::{common::openapi, info::BaseApiUrl};
#[implementation(
    r#trait = InfoHandler,
    name = InfoHandlerImpl,
    result = InfoHandlerResult,
)]
pub mod handler {
    /// Fetch API info
    ///
    /// Returns API info such as version, build date, commit, etc.
    #[openapi(
        responses(
            (status = 200, description = "Returned API info", body = ApiInfoResponse),
        ),
    )]
    #[get("")]
    async fn info(base_api_url: Data<BaseApiUrl>) -> Json<ApiInfoResponse> {
        Json(ApiInfoResponse::new(&base_api_url))
    }
}
