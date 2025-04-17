// Arithmetic
#![deny(arithmetic_overflow)] // Prevent operations that would cause integer overflow
#![deny(clippy::checked_conversions)] // Suggest using checked conversions between numeric types
#![deny(clippy::cast_possible_truncation)] // Detect when casting might truncate a value
#![deny(clippy::cast_sign_loss)] // Detect when casting might lose sign information
#![deny(clippy::cast_possible_wrap)] // Detect when casting might cause value to wrap around
#![deny(clippy::cast_precision_loss)] // Detect when casting might lose precision
#![deny(clippy::integer_division)] // Highlight potential bugs from integer division truncation
#![deny(clippy::arithmetic_side_effects)] // Detect arithmetic operations with potential side effects
#![deny(clippy::unchecked_duration_subtraction)] // Ensure duration subtraction won't cause underflow

// Unwraps
#![deny(clippy::unwrap_used)] // Discourage using .unwrap() which can cause panics
#![deny(clippy::panicking_unwrap)] // Prevent unwrap on values known to cause panics
#![deny(clippy::option_env_unwrap)]
// Prevent unwrapping environment variables which might be absent

// Array indexing
#![deny(clippy::indexing_slicing)]
// Avoid direct array indexing and use safer methods like .get()

// Path handling
#![deny(clippy::join_absolute_paths)]
// Prevent issues when joining paths with absolute paths

// Serialization issues
#![deny(clippy::serde_api_misuse)]
// Prevent incorrect usage of Serde's serialization/deserialization API

// Unbounded input
#![deny(clippy::uninit_vec)]
// Prevent creating uninitialized vectors which is unsafe

// Unsafe code detection
#![deny(clippy::transmute_int_to_char)] // Prevent unsafe transmutation from integers to characters
#![deny(clippy::transmute_int_to_float)] // Prevent unsafe transmutation from integers to floats
#![deny(clippy::transmute_ptr_to_ref)] // Prevent unsafe transmutation from pointers to references
#![deny(clippy::transmute_undefined_repr)] // Detect transmutes with potentially undefined representations

use std::{fmt::Display, sync::Arc};

use ::utils::{
    adapters::{S3, SurrealDB},
    auth::password_hashing::PasswordHasher,
    lgtm::LGTM,
    logger::CustomActixLogger,
};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    web::{Data, FormConfig, JsonConfig, PathConfig, QueryConfig, get},
};
use actix_web_lab::middleware::CatchPanic;
use actix_web_validation::garde::GardeErrorHandlerExt;
use env_vars_config::env_vars_config;
use handler::{
    common::{
        ApiError, ValidationError,
        wrapper::{BaseApiUrl, JwtSecret},
    },
    info::{InfoHandler, implementation::ImplementedInfoHandler},
    profile::{ProfileHandler, implementation::ImplementedProfileHandler},
    review::{ReviewHandler, implementation::ImplementedReviewHandler},
    user::{UserHandler, implementation::ImplementedUserHandler},
};
use repository::{
    image::s3::S3ImageRepository, profile::surreal::SurrealProfileRepository,
    reviewed::surreal::SurrealReviewedRepository,
    user::surreal::SurrealUserRepository,
};
use service::{
    profile::{
        ProfileServiceDependency, implementation::ImplementedProfileService,
    },
    profile_image::{
        ProfileImageServiceDependency,
        implementation::ImplementedProfileImageService,
    },
    reviewed::{
        ReviewedServiceDependency, implementation::ImplementedReviewedService,
    },
    user::{UserServiceDependency, implementation::ImplementedUserService},
};
use utils::{OpenApi, cors::default_cors, validation};
use utoipa::{OpenApi as _, openapi::OpenApi as OpenApiStruct};
use utoipa_actix_web::{AppExt, service_config::ServiceConfig};

pub mod utils;

env_vars_config! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    DB_ADDRESS: String = "localhost:8001",
    DB_NAMESPACE: String = "minizord",
    DB_NAME: String = "api",
    DB_USER: String = "root",
    DB_PASSWORD: String = "root",
    JWT_SECRET: String = "P9mzO6aO64hgkVCBN96CfpUXB1x58XA3zmGuoT4HjSdhHgyRBnqv/EsPDCfs9CRT/oEJYSu6YDcvmdrf/utDNQ==",
    OTEL_ENDPOINT: String = "http://localhost:4317",
    OTEL_SERVICE_NAME: String = "minizord_api",
    S3_BASE_URL: String = "http://localhost:9000",
    S3_ACCESS_KEY: String = "minioadmin",
    S3_SECRET_KEY: String = "minioadmin",
    S3_REGION: String = "custom",
    DEPLOY_DOMAIN: String = "localhost",
    BASE_API_URL: String = "http://localhost:8080",
}

#[derive(Clone)]
struct AppConfig {
    user_service: UserServiceDependency,
    profile_service: ProfileServiceDependency,
    profile_image_service: ProfileImageServiceDependency,
    reviewed_service: ReviewedServiceDependency,
}
impl AppConfig {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(
                FormConfig::default().error_handler(input_err_handler),
            )
            .app_data(PathConfig::default().error_handler(input_err_handler))
            .app_data(QueryConfig::default().error_handler(input_err_handler))
            .app_data(JsonConfig::default().error_handler(input_err_handler))
            .app_data(Data::new(JwtSecret(config::JWT_SECRET.to_owned())))
            .app_data(Data::new(BaseApiUrl(config::BASE_API_URL.to_owned())))
            .configure(ImplementedUserHandler::routes(self.user_service))
            .configure(ImplementedProfileHandler::routes(
                self.profile_service,
                self.profile_image_service,
            ))
            .configure(ImplementedReviewHandler::routes(self.reviewed_service))
            .configure(ImplementedInfoHandler::routes())
            .default_service(get().to(Api::not_found));
        }
    }
}
#[tracing::instrument(skip_all, level = "trace")]
fn input_err_handler<'a, T: Display>(
    err: T,
    _req: &'a HttpRequest,
) -> actix_web::Error {
    ValidationError::with_description(&err.to_string()).into()
}

pub struct Api {
    config: AppConfig,
    openapi: OpenApiStruct,
    lgtm: LGTM,
}
impl Api {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn setup(lgtm: LGTM, db: SurrealDB, s3: S3) -> Self {
        let surreal_client = Arc::new(db);
        let s3_client = Arc::new(s3);

        let user_repository =
            SurrealUserRepository::new(surreal_client.clone());
        let profile_repository =
            SurrealProfileRepository::new(surreal_client.clone());
        let image_repository = S3ImageRepository::new(s3_client.clone());
        let reviewed_repository =
            SurrealReviewedRepository::new(surreal_client.clone());

        let password_hasher = PasswordHasher::new();

        let user_service = ImplementedUserService::new(
            user_repository.clone(),
            config::JWT_SECRET.clone(),
            password_hasher.clone(),
        );
        let profile_service = ImplementedProfileService::new(
            user_repository.clone(),
            profile_repository.clone(),
        );
        let profile_image_service = ImplementedProfileImageService::new(
            image_repository.clone(),
            profile_service.clone(),
        );
        let reviewed_service = ImplementedReviewedService::new(
            reviewed_repository.clone(),
            user_service.clone(),
        );

        Self {
            config: AppConfig {
                user_service,
                profile_service,
                profile_image_service,
                reviewed_service,
            },
            openapi: OpenApi::openapi(),
            lgtm,
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        tracing::info!("Starting the web server");

        HttpServer::new(move || {
            App::new()
                .garde_error_handler(Arc::new(validation::error_handler))
                .wrap(self.lgtm.metrics_middleware())
                .wrap(CatchPanic::default())
                .wrap(default_cors())
                .wrap(LGTM::tracing_middleware())
                .wrap(CustomActixLogger::new())
                .into_utoipa_app()
                .openapi(self.openapi.clone())
                .openapi_service(OpenApi::ui_service)
                .configure(self.config.clone().build())
                .into_app()
        })
        .bind(config::SERVER_ADDRESS.clone())?
        .run()
        .await?;

        tracing::info!("Shutting down the web server");
        Ok(())
    }

    #[tracing::instrument(skip_all, level = "trace")]
    pub async fn not_found() -> HttpResponse {
        HttpResponse::NotFound().json(ApiError {
            error: "not_found".into(),
            description: "The requested route does not exist".into(),
        })
    }
}
