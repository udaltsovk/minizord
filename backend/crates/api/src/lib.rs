use std::sync::Arc;

use ::utils::{
    adapters::{S3, SurrealDB},
    auth::password_hashing::PasswordHasher,
    lgtm::LGTM,
    logger::CustomActixLogger,
};
use actix_web::{
    App, HttpServer,
    web::{Data, FormConfig, JsonConfig, PathConfig, QueryConfig, get},
};
use actix_web_lab::middleware::CatchPanic;
use actix_web_validation::garde::GardeErrorHandlerExt;
use env_vars_config::env_vars_config;
use handler::{
    common::wrapper::{BaseApiUrl, JwtSecret},
    info::{InfoHandler, implementation::InfoHandlerImpl},
    profile::{ProfileHandler, implementation::ProfileHandlerImpl},
    review::{ReviewHandler, implementation::ReviewHandlerImpl},
    user::{UserHandler, implementation::UserHandlerImpl},
};
use repository::{
    image::s3::S3ImageRepository, profile::surreal::SurrealProfileRepository,
    reviewed::surreal::SurrealReviewedRepository,
    user::surreal::SurrealUserRepository,
};
use service::{
    profile::{ProfileServiceDependency, implementation::ProfileServiceImpl},
    profile_image::{
        ProfileImageServiceDependency, implementation::ProfileImageServiceImpl,
    },
    review::{ReviewServiceDependency, implementation::ReviewServiceImpl},
    user::{UserServiceDependency, implementation::UserServiceImpl},
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
    METRICS_ADDRESS: String = "0.0.0.0:8081",
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
    review_service: ReviewServiceDependency,
}
impl AppConfig {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(
                FormConfig::default().error_handler(handler::input_error),
            )
            .app_data(PathConfig::default().error_handler(handler::input_error))
            .app_data(
                QueryConfig::default().error_handler(handler::input_error),
            )
            .app_data(JsonConfig::default().error_handler(handler::input_error))
            .app_data(Data::new(JwtSecret(config::JWT_SECRET.to_owned())))
            .app_data(Data::new(BaseApiUrl(config::BASE_API_URL.to_owned())))
            .configure(UserHandlerImpl::routes(self.user_service))
            .configure(ProfileHandlerImpl::routes(
                self.profile_service,
                self.profile_image_service,
            ))
            .configure(ReviewHandlerImpl::routes(self.review_service))
            .configure(InfoHandlerImpl::routes())
            .default_service(get().to(handler::not_found));
        }
    }
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

        let user_service = UserServiceImpl::new(
            user_repository.clone(),
            config::JWT_SECRET.clone(),
            password_hasher.clone(),
        );
        let profile_service = ProfileServiceImpl::new(
            user_repository.clone(),
            profile_repository.clone(),
        );
        let profile_image_service = ProfileImageServiceImpl::new(
            image_repository.clone(),
            profile_service.clone(),
        );
        let review_service = ReviewServiceImpl::new(
            reviewed_repository.clone(),
            user_service.clone(),
        );

        Self {
            config: AppConfig {
                user_service,
                profile_service,
                profile_image_service,
                review_service,
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
                .wrap(CustomActixLogger::new())
                .wrap(LGTM::tracing_middleware())
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
}
