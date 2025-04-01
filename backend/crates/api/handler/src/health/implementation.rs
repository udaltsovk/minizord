use actix_web::{HttpResponse, get};
use macros::handler_implementation;
use utoipa::path as openapi;

handler_implementation! {
    HealthHandler as Implemented {
        /// API status check
        ///
        /// Verify that API is running and ready to accept incoming requests.
        #[openapi(
            operation_id = "health",
            responses(
                (status = 200, description = "API is running"),
            ),
        )]
        #[get("")]
        health() -> HttpResponse {
            HttpResponse::Ok().finish()
        }
    }
}
