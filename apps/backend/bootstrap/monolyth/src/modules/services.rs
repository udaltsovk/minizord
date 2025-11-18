use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::{
    hasher::argon2::{Argon2AdapterError, Argon2Service},
    token::jwt::{JwtAdapterError, JwtService},
};

#[derive(Clone)]
pub struct ServicesModule {
    hasher_service: Arc<Argon2Service>,
    token_service: Arc<JwtService>,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Argon2 error: {0}")]
    Argon2(#[from] Argon2AdapterError),

    #[error("Jwt service error: {0}")]
    Jwt(#[from] JwtAdapterError),
}

impl ServicesModuleExt for ServicesModule {
    type Error = ServiceError;
    type HasherService = Argon2Service;
    type TokenService = JwtService;

    fn hasher_service(&self) -> Arc<Self::HasherService> {
        self.hasher_service.clone()
    }

    fn token_service(&self) -> Arc<Self::TokenService> {
        self.token_service.clone()
    }
}

#[allow(dead_code)]
impl ServicesModule {
    pub(crate) fn new(jwt_secret: &str) -> Self {
        let hasher_service = Arc::new(Argon2Service::new());
        let token_service = Arc::new(JwtService::new(jwt_secret));

        Self {
            hasher_service,
            token_service,
        }
    }
}
