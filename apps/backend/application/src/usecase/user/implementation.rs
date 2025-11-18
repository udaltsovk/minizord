use async_trait::async_trait;
use domain::user::{RegisterUser, User};
use lib::{
    domain::{DomainType as _, Id},
    instrument_all,
};
use tap::Pipe as _;

use crate::{
    repository::{RepositoriesModuleExt, user::UserRepository},
    service::{ServicesModuleExt, hasher::HasherService as _},
    usecase::{
        UseCase,
        user::{UserUseCase, error::UserUseCaseError},
    },
};

#[async_trait]
#[instrument_all("UserUseCase")]
impl<R, S> UserUseCase<R, S> for UseCase<R, S, User>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn register(
        &self,
        source: RegisterUser,
    ) -> Result<User, UserUseCaseError<R, S>> {
        if self
            .repositories
            .user_repository()
            .exists_by_username(source.username.as_ref())
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
        {
            return UserUseCaseError::UsernameTaken(source.username.into())
                .pipe(Err);
        }

        let password_hash = self
            .services
            .hasher_service()
            .hash(source.password.into_inner().as_bytes())
            .map_err(S::Error::from)
            .map_err(UserUseCaseError::Service)?;

        let user = User {
            id: Id::generate(),
            email: source.email,
            username: source.username,
            password_hash,
            role: source.role,
            profile_id: None,
        };

        self.repositories
            .user_repository()
            .save(user)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
            .pipe(Ok)
    }
}
