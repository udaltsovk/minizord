#![allow(clippy::empty_docs)]

use std::fmt::Display;

use actix_web::{HttpRequest, HttpResponse};
use common::{ApiError, ValidationError};

pub mod common;
pub mod info;
pub mod profile;
pub mod review;
pub mod user;

#[tracing::instrument(skip_all, level = "trace")]
pub fn input_error<T: Display>(err: T, _req: &HttpRequest) -> actix_web::Error {
    ValidationError::with_description(&err.to_string()).into()
}

#[tracing::instrument(skip_all, level = "trace")]
pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ApiError {
        error: "not_found".into(),
        description: "The requested route does not exist".into(),
    })
}
