use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use utoipa::{IntoResponses, ToSchema};

use super::ApiError;

#[derive(thiserror::Error, IntoResponses, ToSchema, Debug)]
pub enum AuthenticationError {
    #[response(status = 401)]
    #[error("Missing Authorization Header")]
    NoAuthorizationHeader,

    #[response(status = 401)]
    #[error("Invalid Authentication Credentials")]
    InvalidCredentials,

    #[response(status = 401)]
    #[error("Authentication method was not valid")]
    InvalidAuthMethod,

    #[response(status = 403)]
    #[error("Missing permissions")]
    MissingPermissions,
}

impl ResponseError for AuthenticationError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn status_code(&self) -> StatusCode {
        use StatusCode as SC;
        match self {
            Self::NoAuthorizationHeader => SC::UNAUTHORIZED,
            Self::InvalidCredentials => SC::UNAUTHORIZED,
            Self::InvalidAuthMethod => SC::UNAUTHORIZED,
            Self::MissingPermissions => SC::FORBIDDEN,
        }
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiError {
            error: self.error_name().to_string(),
            description: self.to_string(),
        })
    }
}

impl AuthenticationError {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn error_name(&self) -> &'static str {
        match self {
            Self::NoAuthorizationHeader => "missing_authorization_header",
            Self::InvalidCredentials => "invalid_credentials",
            Self::InvalidAuthMethod => "invalid_auth_method",
            Self::MissingPermissions => "missing_permissions",
        }
    }
}
