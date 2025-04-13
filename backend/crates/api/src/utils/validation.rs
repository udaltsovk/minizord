use std::collections::HashMap;

use actix_web::HttpRequest;
use garde::Report;
use handler::common::ValidationError as ValidationErrorStruct;

#[tracing::instrument(skip_all, level = "trace")]
pub fn error_handler(report: Report, _req: &HttpRequest) -> actix_web::Error {
    ValidationErrorStruct::with_errors(report.iter().fold(
        HashMap::new(),
        |mut acc, (path, err)| {
            acc.entry(path.to_string())
                .or_default()
                .push(err.clone().to_string());
            acc
        },
    ))
    .into()
}
