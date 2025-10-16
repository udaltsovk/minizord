use std::sync::Arc;

use application::usecase::{
    UseCase, session::SessionUseCase, user::UserUseCase,
};
use domain::{session::Session, user::User};
use include_dir::include_dir;
use infrastructure::persistence::surreal::{ConnectionProtocol, Surreal};
use presentation::api::rest::module::ModulesExt;

use crate::{
    config,
    modules::{repositories::RepositoriesModule, services::ServicesModule},
};

mod repositories;
mod services;

type UseCaseArc<T> = Arc<UseCase<RepositoriesModule, ServicesModule, T>>;

#[derive(Clone)]
pub struct Modules {
    base_api_url: Arc<str>,
    user_usecase: UseCaseArc<User>,
    session_usecase: UseCaseArc<Session>,
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;
    type ServicesModule = ServicesModule;

    fn base_api_url(&self) -> &str {
        &self.base_api_url
    }

    fn user_usecase(
        &self,
    ) -> &impl UserUseCase<Self::RepositoriesModule, Self::ServicesModule> {
        &*self.user_usecase
    }

    fn session_usecase(
        &self,
    ) -> &impl SessionUseCase<Self::RepositoriesModule, Self::ServicesModule>
    {
        &*self.session_usecase
    }
}

impl Modules {
    pub async fn init() -> Self {
        let surreal = Surreal::new(
            &config::SURREAL_ADDRESS,
            &config::SURREAL_NAMESPACE,
            &config::SURREAL_DATABASE,
            &config::SURREAL_USERNAME,
            &config::SURREAL_PASSWORD,
            ConnectionProtocol::Ws,
            *config::SURREAL_MAX_POOL_SIZE,
        )
        .await
        .migrate(&include_dir!(
            "apps/backend/infrastructure/persistence/surreal/db"
        ))
        .await
        .expect("Failed to run migrations");

        let repositories_module = RepositoriesModule::new(&surreal);
        let services_module = ServicesModule::new(&config::JWT_SECRET);

        let user_usecase = Arc::new(UseCase::new(
            repositories_module.clone(),
            services_module.clone(),
        ));

        let session_usecase = Arc::new(UseCase::new(
            repositories_module.clone(),
            services_module.clone(),
        ));

        Self {
            base_api_url: config::BASE_API_URL.clone().into(),
            user_usecase,
            session_usecase,
        }
    }
}
