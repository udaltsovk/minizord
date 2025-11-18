use std::fmt::Debug;

use axum::{extract::rejection::JsonRejection, http::StatusCode};
use domain::error::DomainError;
use lib::domain::validation::error::ValidationErrors;

mod usecase;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error("{error}")]
    UseCase {
        status_code: StatusCode,
        error_code: String,
        error: String,
    },
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        use DomainError as DE;
        match err {
            DE::Validation(err) => Self::Validation(err),
        }
    }
}
