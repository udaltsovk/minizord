use actix_web::{HttpResponse, get};
use macros::handler_implementation;
use utoipa::path as openapi;

handler_implementation! {
    HealthHandler as Implemented {
        #[openapi(
            tag = "Health",
            operation_id = "health",
            responses(
                (status = 200, description = ""),
            ),
        )]
        #[get("")]
        ///
        ///
        ///
        health() -> HttpResponse {
            HttpResponse::Ok().into()
        }

        #[openapi(
            tag = "Health",
            operation_id = "ping",
            responses(
                (status = 200, description = ""),
            ),
        )]
        #[get("/ping")]
        ///
        ///
        ///
        ping() -> HttpResponse {
            HttpResponse::NotImplemented().into()
        }
    }
}
