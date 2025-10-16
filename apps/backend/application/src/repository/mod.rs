use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::repository::user::UserRepository;

pub mod user;

pub trait RepositoriesModuleExt: Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::UserRepo as UserRepository>::AdapterError>;

    type UserRepo: UserRepository + Send + Sync;

    fn user_repository(&self) -> Arc<Self::UserRepo>;
}
