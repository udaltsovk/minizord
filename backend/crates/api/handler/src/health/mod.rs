use actix_web::HttpResponse;
use macros::handler;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub mod implementation;

handler! {
    Health {
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
