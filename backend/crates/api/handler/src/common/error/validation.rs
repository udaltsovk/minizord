use std::{borrow::Cow, collections::HashMap, fmt::Display};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;
use validator::ValidationError as OriginalValidationError;

///
#[derive(Serialize, ToSchema, Debug)]
pub struct ValidationErrorFieldError {
    ///
    message: String,

    ///
    #[schema(inline)]
    params: HashMap<Cow<'static, str>, Value>,
}
impl From<OriginalValidationError> for ValidationErrorFieldError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(err: OriginalValidationError) -> Self {
        Self {
            message: err.code.to_string(),
            params: err.params,
        }
    }
}

type ValidationErrorErrors = HashMap<String, Vec<ValidationErrorFieldError>>;

///
#[derive(Serialize, ToSchema, Debug)]
pub struct ValidationError {
    ///
    pub error: String,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(inline)]
    pub errors: Option<ValidationErrorErrors>,
}
impl Display for ValidationError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}
impl ValidationError {
    pub fn new(
        description: Option<&str>,
        errors: Option<ValidationErrorErrors>,
    ) -> Self {
        Self {
            error: "invalid_input".into(),
            description: description.map(|d| d.to_string()),
            errors,
        }
    }

    pub fn with_description(description: &str) -> Self {
        Self::new(Some(description), None)
    }

    pub fn with_errors(errors: ValidationErrorErrors) -> Self {
        Self::new(None, Some(errors))
    }
}
impl ResponseError for ValidationError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn status_code(&self) -> StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
