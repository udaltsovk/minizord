use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error("{0}")]
    Service(S::Error),

    #[error("Username `{0}` is already taken")]
    UsernameTaken(String),

    #[error("User with username `{0}` does not exist")]
    NotFound(String),
}
