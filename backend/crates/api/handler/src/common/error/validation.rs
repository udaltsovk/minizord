use std::{collections::HashMap, fmt::Display};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

type ValidationErrorErrors = HashMap<String, Vec<String>>;

#[derive(Serialize, ToSchema, IntoResponses, Debug)]
#[response(status = 400)]
pub struct ValidationError {
    ///
    pub error: String,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(inline)]
    pub field_errors: Option<ValidationErrorErrors>,
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
            field_errors: errors,
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
