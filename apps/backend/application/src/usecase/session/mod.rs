use async_trait::async_trait;
use domain::{
    session::{
        SessionTokenPair, VerboseSession, refresh_token::SessionRefreshToken,
    },
    user::{password::UserPassword, username::Username},
};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::session::error::SessionUseCaseError,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait SessionUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        username: Username,
        password: UserPassword,
    ) -> Result<VerboseSession, SessionUseCaseError<R, S>>;

    async fn refresh(
        &self,
        refresh_token: SessionRefreshToken,
    ) -> Result<SessionTokenPair, SessionUseCaseError<R, S>>;
}
