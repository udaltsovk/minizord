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

    #[response(status = 413)]
    #[error("Payload size exceeds the limit of {0}")]
    PayloadTooLarge(String),

    #[response(status = 415)]
    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),

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
            SE::PayloadTooLarge(msg) => Self::PayloadTooLarge(msg),
            SE::UnsupportedMediaType {
                ..
            } => Self::UnsupportedMediaType(err.to_string()),
            SE::Hasher(msg) => Self::Internal(msg.to_string()),
            SE::Database(msg) | SE::Internal(msg) => Self::Internal(msg),
        }
    }
}
impl HandlerError {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn as_api_error(&self) -> ApiError {
        ApiError {
            error: match self {
                Self::Authentication(err) => err.error_name(),
                Self::Unauthorized(..) => "unauthorized",
                Self::Forbidden | Self::ForbiddenWithMsg(..) => "access_denied",
                Self::NotFound(..) => "not_found",
                Self::AlreadyExists(..) => "already_exists",
                Self::PayloadTooLarge(..) => "payload_too_large",
                Self::UnsupportedMediaType {
                    ..
                } => "unsupported_media_type",
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
            Self::Authentication(err) => err.status_code(),
            Self::Unauthorized(..) => SC::UNAUTHORIZED,
            Self::Forbidden | Self::ForbiddenWithMsg(..) => SC::FORBIDDEN,
            Self::NotFound(..) => SC::NOT_FOUND,
            Self::AlreadyExists(..) => SC::CONFLICT,
            Self::PayloadTooLarge(..) => SC::PAYLOAD_TOO_LARGE,
            Self::UnsupportedMediaType {
                ..
            } => SC::UNSUPPORTED_MEDIA_TYPE,
            Self::Internal(..) => SC::INTERNAL_SERVER_ERROR,
        }
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
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
