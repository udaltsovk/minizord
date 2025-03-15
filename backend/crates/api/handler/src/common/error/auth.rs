use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

use super::ApiError;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Missing Authorization Header")]
    NoAuthorizationHeader,

    #[error("Invalid Authentication Credentials")]
    InvalidCredentials,

    #[error("Authentication method was not valid")]
    InvalidAuthMethod,

    #[error("Missing permissions")]
    MissingPermissions,

    #[error("User email/account is already registered")]
    DuplicateUser,
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        use StatusCode as SC;
        match self {
            Self::NoAuthorizationHeader => SC::UNAUTHORIZED,
            Self::InvalidCredentials => SC::UNAUTHORIZED,
            Self::InvalidAuthMethod => SC::UNAUTHORIZED,
            Self::MissingPermissions => SC::FORBIDDEN,
            Self::DuplicateUser => SC::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiError {
            error: self.error_name().to_string(),
            description: self.to_string(),
        })
    }
}

impl AuthenticationError {
    pub fn error_name(&self) -> &'static str {
        match self {
            Self::NoAuthorizationHeader => "missing_authorization_header",
            Self::InvalidCredentials => "invalid_credentials",
            Self::InvalidAuthMethod => "invalid_auth_method",
            Self::MissingPermissions => "missing_permissions",
            Self::DuplicateUser => "duplicate_user",
        }
    }
}
