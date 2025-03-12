use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use service::common::ServiceError;
use utoipa::ToSchema;

#[derive(thiserror::Error, Debug)]
pub enum HandlerError {
    #[error("{0}")]
    Database(String),
}

impl From<ServiceError> for HandlerError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Database(msg) => Self::Database(msg),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ApiError {
    pub error: String,
    pub description: String,
}

impl HandlerError {
    pub fn as_api_error(&self) -> ApiError {
        ApiError {
            error: match self {
                Self::Database(..) => "database_error",
            }
            .to_string(),
            description: self.to_string(),
        }
    }
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
