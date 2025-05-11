use actix_web::{
    get,
    web::{Data, Json},
};
use macros::handler_implementation;
use tracing::instrument;

use super::{
    ApiInfoResponse, InfoHandler, InfoHandlerHelper, InfoHandlerResult,
};
use crate::{common::openapi, info::BaseApiUrl};

handler_implementation! {
    InfoHandler as InfoHandlerImpl {
        /// Fetch API info
        ///
        /// Returns API info such as version, build date, commit, etc.
        #[openapi(
            responses(
                (status = 200, description = "Returned API info", body = ApiInfoResponse),
            ),
        )]
        #[get("")]
        #[instrument(skip_all, name = "InfoHandler::info")]
        async fn info(
            base_api_url: Data<BaseApiUrl>,
        ) ->Json<ApiInfoResponse> {
            Json(ApiInfoResponse::new(&base_api_url.0))
        }
    }
}
