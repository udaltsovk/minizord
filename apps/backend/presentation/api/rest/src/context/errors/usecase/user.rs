use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseError,
};
use axum::http::StatusCode;

use crate::context::errors::AppError;

impl<R, S> From<UserUseCaseError<R, S>> for AppError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: UserUseCaseError<R, S>) -> Self {
        use StatusCode as C;
        use UserUseCaseError as E;

        let (status_code, error_code) = match error {
            E::Repository(_) | E::Service(_) => {
                (C::INTERNAL_SERVER_ERROR, "internal_server_error")
            },
            E::UsernameTaken(_) => (C::CONFLICT, "username_taken"),
            E::NotFound(_) => (C::NOT_FOUND, "user_not_found"),
        };

        Self::UseCase {
            status_code,
            error_code: error_code.to_string(),
            error: error.to_string(),
        }
    }
}
