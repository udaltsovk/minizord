#![allow(clippy::empty_docs)]
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use service::common::ServiceError;
use utoipa::{IntoResponses, ToSchema};

pub mod auth;
pub mod validation;

///
#[derive(thiserror::Error, IntoResponses, ToSchema, Debug)]
#[schema(as = ApiError)]
pub enum HandlerError {
    #[response(status = 400)]
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[response(status = "default")]
    #[error("Authentication error: {0}")]
    Authentication(
        #[from]
        #[schema(inline)]
        auth::AuthenticationError,
    ),

    #[response(status = 401)]
    #[error("Authentication error: {0}")]
    Unauthorized(String),

    #[response(status = 403)]
    #[error("Access denied")]
    Forbidden,

    #[response(status = 403)]
    #[error("{0}")]
    ForbiddenWithMsg(String),

    #[response(status = 404)]
    #[error("{0} was not found")]
    NotFound(String),

    #[response(status = 409)]
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[response(status = 500)]
    #[error("{0}")]
    Internal(String),
}

impl From<ServiceError> for HandlerError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(err: ServiceError) -> Self {
        use ServiceError as SE;
        match err {
            SE::InvalidPassword => Self::Unauthorized(err.to_string()),
            SE::Forbidden(msg) => Self::ForbiddenWithMsg(msg),
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
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn as_api_error(&self) -> ApiError {
        ApiError {
            error: match self {
                Self::InvalidInput(..) => "invalid_input",
                Self::Authentication(err) => err.error_name(),
                Self::Unauthorized(..) => "unauthorized",
                Self::Forbidden | Self::ForbiddenWithMsg(..) => "access_denied",
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
    #[tracing::instrument(skip_all, level = "trace")]
    fn status_code(&self) -> StatusCode {
        use StatusCode as SC;
        match self {
            Self::InvalidInput(..) => SC::BAD_REQUEST,
            Self::Authentication(err) => err.status_code(),
            Self::Unauthorized(..) => SC::UNAUTHORIZED,
            Self::Forbidden | Self::ForbiddenWithMsg(..) => SC::FORBIDDEN,
            Self::NotFound(..) => SC::NOT_FOUND,
            Self::AlreadyExists(..) => SC::CONFLICT,
            Self::Internal(..) => SC::INTERNAL_SERVER_ERROR,
        }
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
