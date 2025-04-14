use actix_web::web::Json;
use macros::{handler, response};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::common::HandlerError;

pub mod implementation;

handler! {
    Info
        Err: HandlerError
    {
        #routes() {
            move |cfg: &mut ServiceConfig| {
                cfg.service(scope("/")
                    .service(Self::info())
                );
            }
        }

        info() -> Json<ApiInfoResponse>;
    }
}

response! {
    ///
    ApiInfo {
        ///
        #[schema(examples("minizord-api"))]
        name: &'static str,

        ///
        #[schema(examples("0.1.0"))]
        version: &'static str,

        ///
        #[schema(format = Uri)]
        documentation: &'static str,

        ///
        #[schema(examples("Welcome, traveler!"))]
        about: &'static str,

        ///
        build_info: BuildInfoResponse,
    }

    ///
    BuildInfo {
        ///
        #[schema(examples("1970-01-01 @ 03:00 (Europe/Moscow)"))]
        comp_date: &'static str,

        ///
        git_hash: &'static str,

        ///
        #[schema(examples("release"))]
        profile: &'static str,
    }
}

impl ApiInfoResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let profile = env!("COMPILATION_PROFILE");
        Self {
            name: "minizord-api",
            version: env!("CARGO_PKG_VERSION"),
            documentation: env!("DOCUMENTATION_URL"),
            about: "Welcome, traveler!",
            build_info: BuildInfoResponse {
                comp_date: env!("COMPILATION_DATE"),
                git_hash: env!("GIT_HASH", "unknown"),
                profile,
            },
        }
    }
}
