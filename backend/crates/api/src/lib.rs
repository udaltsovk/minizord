use std::sync::Arc;

use ::utils::{
    LGTM,
    adapters::{S3, SurrealPool},
    auth::PasswordHasher,
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
    profile::{
        ProfileService, ProfileServiceDependency,
        implementation::ProfileServiceImpl,
    },
    profile_image::{
        ProfileImageServiceDependency, implementation::ProfileImageServiceImpl,
    },
    review::{
        ReviewService, ReviewServiceDependency,
        implementation::ReviewServiceImpl,
    },
    user::{
        UserService, UserServiceDependency, implementation::UserServiceImpl,
    },
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
    DB_MAX_POOL_SIZE: u64 = 16u64,
    JWT_SECRET: String = "P9mzO6aO64hgkVCBN96CfpUXB1x58XA3zmGuoT4HjSdhHgyRBnqv/EsPDCfs9CRT/oEJYSu6YDcvmdrf/utDNQ==",
    METRICS_ADDRESS: String = "0.0.0.0:8081",
    OTEL_ENDPOINT: String = "http://localhost:4317",
    S3_BASE_URL: String = "http://localhost:9000",
    S3_ACCESS_KEY: String = "minioadmin",
    S3_SECRET_KEY: String = "minioadmin",
    S3_REGION: String = "custom",
    DEPLOY_DOMAIN: String = "localhost",
    BASE_API_URL: String = "http://localhost:8080",
}

#[derive(Clone)]
struct AppConfig {
    base_api_url: BaseApiUrl,
    jwt_secret: JwtSecret,
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
            .app_data(Data::new(self.base_api_url.clone()))
            .app_data(Data::new(self.jwt_secret.clone()))
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
    pub async fn setup(lgtm: LGTM, db: SurrealPool, s3: S3) -> Self {
        let user_repository = SurrealUserRepository::new(db.clone());
        let profile_repository = SurrealProfileRepository::new(db.clone());
        let image_repository = S3ImageRepository::new(s3.clone());
        let reviewed_repository = SurrealReviewedRepository::new(db.clone());

        let base_api_url = BaseApiUrl::from(config::BASE_API_URL.to_owned());
        let jwt_secret = JwtSecret::from(config::JWT_SECRET.to_owned());
        let password_hasher = PasswordHasher::new();

        let user_service = UserServiceImpl::new(
            user_repository.clone(),
            jwt_secret.clone(),
            password_hasher.clone(),
        );
        let profile_service = ProfileServiceImpl::new(
            user_repository.clone(),
            profile_repository.clone(),
            user_service.clone(),
        );
        let profile_image_service = ProfileImageServiceImpl::new(
            image_repository.clone(),
            profile_service.clone(),
        );
        let review_service = ReviewServiceImpl::new(
            reviewed_repository.clone(),
            user_service.clone(),
        );

        user_service.init_metrics().await;
        profile_service.init_metrics().await;
        review_service.init_metrics().await;

        Self {
            config: AppConfig {
                jwt_secret,
                base_api_url,
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
