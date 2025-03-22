use ::utils::auth::password_hashing::PasswordHasher;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, FormConfig, JsonConfig, PathConfig, QueryConfig, get},
};
use env_vars_config::env_vars_config;
use handler::{
    common::{ApiError, HandlerError},
    health::{HealthHandler, implementation::ImplementedHealthHandler},
    mentor::{MentorHandler, implementation::ImplementedMentorHandler},
    organizator::{OrganizatorHandler, implementation::ImplementedOrganizatorHandler},
    participant::{ParticipantHandler, implementation::ImplementedParticipantHandler},
};
use repository::{
    common::adapters::surrealdb::SurrealDB, mentor::surreal::SurrealMentorRepository,
    organizator::surreal::SurrealOrganizatorRepository,
    participant::surreal::SurrealParticipantRepository,
};
use service::{
    mentor::{MentorServiceDependency, implementation::ImplementedMentorService},
    organizator::{OrganizatorServiceDependency, implementation::ImplementedOrganizatorService},
    participant::{ParticipantServiceDependency, implementation::ImplementedParticipantService},
};
use std::{fmt::Display, sync::Arc};
use utils::openapi::OpenApiVisualiser;
use utoipa::{OpenApi, openapi::OpenApi as OpenApiStruct};
use utoipa_actix_web::service_config::ServiceConfig;

pub mod utils;

env_vars_config! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    DB_ADDRESS: String = "localhost:8001",
    DB_NAMESPACE: String = "megazord",
    DB_NAME: String = "api",
    DB_USER: String = "root",
    DB_PASSWORD: String = "root",
    JWT_SECRET: String = "ohrfwahl;fhjjhawefhjaewfjhhjawfjbklbjlhjeawfjhjhwarjhjhhawhfhjhjfwahl",
    OTLP_ENDPOINT: String = "http://localhost:4317",
    // MINIO_BASE_URL: String = "http://localhost:9000",
    // MINIO_USER: String = "root",
    // MINIO_PASSWORD: String = "beetroot",
    // MINIO_BUCKET: String = "ad-platform-backend-bucket",
}

pub fn app_setup(db: SurrealDB) -> BackendConfig {
    config::init();
    let surreal_client = Arc::new(db);

    let organizator_repository = SurrealOrganizatorRepository::new(surreal_client.clone());
    let mentor_repository = SurrealMentorRepository::new(surreal_client.clone());
    let participant_repository = SurrealParticipantRepository::new(surreal_client.clone());

    let password_hasher = PasswordHasher::new();

    BackendConfig {
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
        openapi: OpenApiVisualiser::openapi(),
    }
}

#[derive(Clone)]
pub struct BackendConfig {
    organizator_service: OrganizatorServiceDependency,
    mentor_service: MentorServiceDependency,
    participant_service: ParticipantServiceDependency,
    pub openapi: OpenApiStruct,
}

#[tracing::instrument(skip_all)]
fn input_err_handler<'a, T: Display>(err: T, _req: &'a HttpRequest) -> actix_web::Error {
    HandlerError::Validation(err.to_string()).into()
}

impl BackendConfig {
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(FormConfig::default().error_handler(input_err_handler))
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
                .default_service(get().to(not_found));
        }
    }
}

#[tracing::instrument]
async fn not_found() -> HttpResponse {
    let data = ApiError {
        error: "not_found".into(),
        description: "the requested route does not exist".into(),
    };

    HttpResponse::NotFound().json(data)
}
