use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use service::common::ServiceError;
use utoipa::ToSchema;

pub mod auth;

#[derive(thiserror::Error, Debug)]
pub enum HandlerError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(#[from] auth::AuthenticationError),

    #[error("Authentication error: {0}")]
    Unauthorized(String),

    #[error("Access denied")]
    Forbidden,

    #[error("{0} was not found")]
    NotFound(String),

    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error("{0}")]
    Internal(String),
}

impl From<ServiceError> for HandlerError {
    fn from(err: ServiceError) -> Self {
        use ServiceError as SE;
        match err {
            SE::InvalidPassword => Self::Unauthorized(err.to_string()),
            SE::NotFound(msg) => Self::NotFound(msg),
            SE::AlreadyExists(msg) => Self::AlreadyExists(msg),
            SE::Hasher(msg) => Self::Internal(msg.to_string()),
            SE::Database(msg) => Self::Internal(msg),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
///
pub struct ApiError {
    ///
    pub error: String,
    ///
    pub description: String,
}

impl HandlerError {
    pub fn as_api_error(&self) -> ApiError {
        ApiError {
            error: match self {
                Self::InvalidInput(..) => "invalid_input",
                Self::Validation(..) => "invalid_input",
                Self::Authentication(err) => err.error_name(),
                Self::Unauthorized(..) => "unauthorized",
                Self::Forbidden => "access_denied",
                Self::NotFound(..) => "not_found",
                Self::AlreadyExists(..) => "already_exists",
                Self::Internal(..) => "internal_error",
            }
            .to_string(),
            description: self.to_string(),
        }
    }
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> StatusCode {
        use StatusCode as SC;
        match self {
            Self::InvalidInput(..) => SC::BAD_REQUEST,
            Self::Validation(..) => SC::BAD_REQUEST,
            Self::Authentication(err) => err.status_code(),
            Self::Unauthorized(..) => SC::UNAUTHORIZED,
            Self::Forbidden => SC::FORBIDDEN,
            Self::NotFound(..) => SC::NOT_FOUND,
            Self::AlreadyExists(..) => SC::CONFLICT,
            Self::Internal(..) => SC::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
