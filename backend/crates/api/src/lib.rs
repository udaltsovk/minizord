use std::{fmt::Display, sync::Arc};

use ::utils::auth::password_hashing::PasswordHasher;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    middleware::{Compress, NormalizePath, TrailingSlash},
    web::{Data, FormConfig, JsonConfig, PathConfig, QueryConfig, get},
};
use actix_web_lab::middleware::CatchPanic;
use actix_web_validation::validator::ValidatorErrorHandlerExt;
use env_vars_config::env_vars_config;
use handler::{
    common::{ApiError, ValidationError},
    health::{HealthHandler, implementation::ImplementedHealthHandler},
    mentor::{MentorHandler, implementation::ImplementedMentorHandler},
    organizator::{
        OrganizatorHandler, implementation::ImplementedOrganizatorHandler,
    },
    participant::{
        ParticipantHandler, implementation::ImplementedParticipantHandler,
    },
};
use repository::{
    common::adapters::surrealdb::SurrealDB,
    mentor::surreal::SurrealMentorRepository,
    organizator::surreal::SurrealOrganizatorRepository,
    participant::surreal::SurrealParticipantRepository,
};
use service::{
    mentor::{
        MentorServiceDependency, implementation::ImplementedMentorService,
    },
    organizator::{
        OrganizatorServiceDependency,
        implementation::ImplementedOrganizatorService,
    },
    participant::{
        ParticipantServiceDependency,
        implementation::ImplementedParticipantService,
    },
};
use utils::{lgtm::LGTM, logger::CustomLogger, openapi::OpenApi, validation};
use utoipa::{OpenApi as _, openapi::OpenApi as OpenApiStruct};
use utoipa_actix_web::{AppExt, service_config::ServiceConfig};

pub mod utils;

env_vars_config! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    DB_ADDRESS: String = "localhost:8001",
    DB_NAMESPACE: String = "megazord",
    DB_NAME: String = "api",
    DB_USER: String = "root",
    DB_PASSWORD: String = "root",
    JWT_SECRET: String = "ohrfwahl;fhjjhawefhjaewfjhhjawfjbklbjlhjeawfjhjhwarjhjhhawhfhjhjfwahl",
    OTEL_ENDPOINT: String = "http://localhost:4317",
    OTEL_SERVICE_NAME: String = "megazord_api",
    // MINIO_BASE_URL: String = "http://localhost:9000",
    // MINIO_USER: String = "root",
    // MINIO_PASSWORD: String = "beetroot",
    // MINIO_BUCKET: String = "ad-platform-backend-bucket",
}

#[derive(Clone)]
struct AppConfig {
    organizator_service: OrganizatorServiceDependency,
    mentor_service: MentorServiceDependency,
    participant_service: ParticipantServiceDependency,
    pub openapi: OpenApiStruct,
}

pub struct Api {
    config: AppConfig,
}
impl Api {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn setup(db: SurrealDB) -> Self {
        let surreal_client = Arc::new(db);

        let organizator_repository =
            SurrealOrganizatorRepository::new(surreal_client.clone());
        let mentor_repository =
            SurrealMentorRepository::new(surreal_client.clone());
        let participant_repository =
            SurrealParticipantRepository::new(surreal_client.clone());

        let password_hasher = PasswordHasher::new();

        Self {
            config: AppConfig {
                organizator_service: ImplementedOrganizatorService::new(
                    organizator_repository,
                    config::JWT_SECRET.clone(),
                    password_hasher.clone(),
                ),
                mentor_service: ImplementedMentorService::new(
                    mentor_repository,
                    config::JWT_SECRET.clone(),
                    password_hasher.clone(),
                ),
                participant_service: ImplementedParticipantService::new(
                    participant_repository,
                    config::JWT_SECRET.clone(),
                    password_hasher.clone(),
                ),
                openapi: OpenApi::openapi(),
            },
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        log::info!("Starting the web server");
        let config = self.config.clone();
        HttpServer::new(move || {
            App::new()
                .validator_error_handler(Arc::new(validation::error_handler))
                .wrap(LGTM::metrics_middleware())
                .wrap(CatchPanic::default())
                .wrap(Compress::default())
                .wrap(NormalizePath::new(if cfg!(feature = "swagger") {
                    TrailingSlash::MergeOnly
                } else {
                    TrailingSlash::Trim
                }))
                .wrap(LGTM::tracing_middleware())
                .wrap(CustomLogger::new())
                .into_utoipa_app()
                .openapi(config.openapi.clone())
                .configure(config.clone().build())
                .openapi_service(OpenApi::ui_service)
                .into_app()
        })
        .bind(config::SERVER_ADDRESS.clone())?
        .run()
        .await?;

        log::info!("Shutting down web server");
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

#[tracing::instrument(skip_all, level = "trace")]
fn input_err_handler<'a, T: Display>(
    err: T,
    _req: &'a HttpRequest,
) -> actix_web::Error {
    ValidationError::with_description(&err.to_string()).into()
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
            .app_data(Data::new(config::JWT_SECRET.to_string()))
            .configure(ImplementedHealthHandler::routes())
            .configure(ImplementedOrganizatorHandler::routes(
                self.organizator_service.clone(),
            ))
            .configure(ImplementedMentorHandler::routes(
                self.mentor_service.clone(),
            ))
            .configure(ImplementedParticipantHandler::routes(
                self.participant_service.clone(),
            ))
            .default_service(get().to(Api::not_found));
        }
    }
}
