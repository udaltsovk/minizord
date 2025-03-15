use actix_web::{
    HttpResponse,
    web::{Data, JsonConfig, PathConfig, get},
};
use env_vars_config::env_vars_config;
use handler::{
    common::{ApiError, HandlerError},
    organizator::{OrganizatorHandler, implementation::ImplementedOrganizatorHandler},
};
use repository::{
    common::adapters::surrealdb::SurrealDB, organizator::surreal::SurrealOrganizatorRepository,
};
use service::organizator::{
    OrganizatorServiceDependency, implementation::ImplementedOrganizatorService,
};
use std::sync::Arc;
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
    // MINIO_BASE_URL: String = "http://localhost:9000",
    // MINIO_USER: String = "root",
    // MINIO_PASSWORD: String = "beetroot",
    // MINIO_BUCKET: String = "ad-platform-backend-bucket",
}

pub fn app_setup(db: SurrealDB) -> BackendConfig {
    config::init();
    let surreal_client = Arc::new(db);
    let organizator_repository = SurrealOrganizatorRepository::new(surreal_client.clone());

    BackendConfig {
        organizator_service: ImplementedOrganizatorService::new(
            organizator_repository.clone(),
            config::JWT_SECRET.to_string(),
        ),
        openapi: OpenApiVisualiser::openapi(),
    }
}

#[derive(Clone)]
pub struct BackendConfig {
    pub organizator_service: OrganizatorServiceDependency,
    pub openapi: OpenApiStruct,
}

impl BackendConfig {
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(
                PathConfig::default()
                    .error_handler(|err, _req| HandlerError::Validation(err.to_string()).into()),
            )
            .app_data(
                JsonConfig::default()
                    .error_handler(|err, _req| HandlerError::Validation(err.to_string()).into()),
            )
            .app_data(Data::new(config::JWT_SECRET.to_string()))
            .configure(ImplementedOrganizatorHandler::routes(
                self.organizator_service.clone(),
            ))
            .default_service(get().to(not_found));
        }
    }
}

async fn not_found() -> HttpResponse {
    let data = ApiError {
        error: "not_found".into(),
        description: "the requested route does not exist".into(),
    };

    HttpResponse::NotFound().json(data)
}
