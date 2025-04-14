use actix_web::{get, web::Json};
use macros::handler_implementation;

use super::{
    ApiInfoResponse, InfoHandler, InfoHandlerHelper, InfoHandlerResult,
};
use crate::common::openapi;

handler_implementation! {
    InfoHandler as Implemented {
        /// Fetch API info
        ///
        /// Returns API info such as version, build date, commit, etc.
        #[openapi(
            operation_id = "info",
            responses(
                (status = 200, description = "Returned API info", body = ApiInfoResponse),
            ),
        )]
        #[get("")]
        info() -> Json<ApiInfoResponse> {
            Json(ApiInfoResponse::new())
        }
    }
}
