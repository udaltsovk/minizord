use actix_web::HttpResponse;
use macros::handler;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::HandlerError;

pub mod implementation;

handler! {
    Health
        Err: HandlerError
    {
        #routes() {
            move |cfg: &mut ServiceConfig| {
                cfg.service(scope("/health")
                    .service(Self::health())
                );
            }
        }

        health() -> HttpResponse;
    }
}
