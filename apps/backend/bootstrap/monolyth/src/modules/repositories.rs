use std::sync::Arc;

use application::repository::RepositoriesModuleExt;
use domain::user::User;
use infrastructure::persistence::surreal::{
    Surreal, error::SurrealAdapterError, repository::SurrealRepositoryImpl,
};

#[derive(Clone)]
pub struct RepositoriesModule {
    user_repository: Arc<SurrealRepositoryImpl<User>>,
}

impl RepositoriesModule {
    pub fn new(surreal: &Surreal) -> Self {
        let user_repository = Arc::new(SurrealRepositoryImpl::new(surreal));

        Self {
            user_repository,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("{0}")]
    Surreal(#[from] SurrealAdapterError),
}

impl RepositoriesModuleExt for RepositoriesModule {
    type Error = RepositoryError;
    type UserRepo = SurrealRepositoryImpl<User>;

    fn user_repository(&self) -> Arc<Self::UserRepo> {
        self.user_repository.clone()
    }
}
