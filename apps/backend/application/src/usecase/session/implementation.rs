use async_trait::async_trait;
use domain::{
    session::{
        Session, SessionTokenPair, VerboseSession,
        refresh_token::SessionRefreshToken,
    },
    user::{password::UserPassword, username::Username},
};
use lib::{
    domain::{DomainType as _, Id},
    instrument_all,
};
use tap::Pipe as _;

use crate::{
    repository::{RepositoriesModuleExt, user::UserRepository as _},
    service::{
        ServicesModuleExt, hasher::HasherService as _, token::TokenService as _,
    },
    usecase::{
        UseCase,
        session::{SessionUseCase, error::SessionUseCaseError},
    },
};

#[async_trait]
#[instrument_all("SessionUseCase")]
impl<R, S> SessionUseCase<R, S> for UseCase<R, S, Session>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        username: Username,
        password: UserPassword,
    ) -> Result<VerboseSession, SessionUseCaseError<R, S>> {
        let user = self
            .repositories
            .user_repository()
            .find_by_username(username.as_ref())
            .await
            .map_err(R::Error::from)
            .map_err(SessionUseCaseError::Repository)?
            .ok_or_else(|| {
                SessionUseCaseError::UserNotFound(username.into())
            })?;

        self.services
            .hasher_service()
            .verify(password.into_inner().as_bytes(), &user.password_hash)
            .map_err(|_| SessionUseCaseError::InvalidCredentials)?;

        let session_id = Id::<Session>::generate();

        let token_pair = self
            .services
            .token_service()
            .generate_pair(session_id)
            .map_err(S::Error::from)
            .map_err(SessionUseCaseError::Service)?;

        VerboseSession {
            id: session_id,
            token_pair,
            user,
        }
        .pipe(Ok)
    }

    async fn refresh(
        &self,
        refresh_token: SessionRefreshToken,
    ) -> Result<SessionTokenPair, SessionUseCaseError<R, S>> {
        let token_service = self.services.token_service();

        let session_id = token_service
            .parse(refresh_token.as_ref())
            .map_err(|_| SessionUseCaseError::InvalidToken)?;

        token_service
            .generate_pair(session_id)
            .map_err(S::Error::from)
            .map_err(SessionUseCaseError::Service)
    }
}
