use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::service::{hasher::HasherService, token::TokenService};

pub mod hasher;
pub mod token;

pub trait ServicesModuleExt: Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::HasherService as HasherService>::AdapterError>
        + From<<Self::TokenService as TokenService>::AdapterError>;

    type HasherService: HasherService + Send + Sync;
    type TokenService: TokenService + Send + Sync;

    fn hasher_service(&self) -> Arc<Self::HasherService>;

    fn token_service(&self) -> Arc<Self::TokenService>;
}
