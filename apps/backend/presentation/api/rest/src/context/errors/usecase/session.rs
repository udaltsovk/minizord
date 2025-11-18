use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::session::error::SessionUseCaseError,
};
use axum::http::StatusCode;

use crate::context::errors::AppError;

impl<R, S> From<SessionUseCaseError<R, S>> for AppError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: SessionUseCaseError<R, S>) -> Self {
        use SessionUseCaseError as E;
        use StatusCode as C;

        let (status_code, error_code) = match error {
            E::Repository(_) | E::Service(_) => {
                (C::INTERNAL_SERVER_ERROR, "internal_server_error")
            },
            E::UserNotFound(_) => (C::NOT_FOUND, "user_not_found"),
            E::InvalidCredentials => (C::UNAUTHORIZED, "invalid_credentials"),
            E::InvalidToken => (C::UNAUTHORIZED, "invalid_token"),
        };

        Self::UseCase {
            status_code,
            error_code: error_code.to_string(),
            error: error.to_string(),
        }
    }
}
