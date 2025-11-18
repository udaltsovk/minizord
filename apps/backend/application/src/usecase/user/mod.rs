use async_trait::async_trait;
use domain::user::{RegisterUser, User};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseError,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait UserUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn register(
        &self,
        source: RegisterUser,
    ) -> Result<User, UserUseCaseError<R, S>>;
}
