use std::collections::HashMap;

use actix_web::HttpRequest;
use handler::common::ValidationError as ValidationErrorStruct;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[tracing::instrument(skip_all, level = "trace")]
pub fn error_handler(
    errors: ValidationErrors,
    _req: &HttpRequest,
) -> actix_web::Error {
    ValidationErrorStruct::with_errors(flatten_errors(&errors).iter().fold(
        HashMap::new(),
        |mut acc, (_, field, err)| {
            if let Some(errs) = acc.get_mut(field) {
                errs.push((**err).clone().into());
            } else {
                acc.insert(field.to_string(), vec![(**err).clone().into()]);
            }
            acc
        },
    ))
    .into()
}

#[inline]
fn flatten_errors(
    errors: &ValidationErrors,
) -> Vec<(u16, String, &ValidationError)> {
    _flatten_errors(errors, None, None)
}

#[inline]
fn _flatten_errors(
    errors: &ValidationErrors,
    path: Option<String>,
    indent: Option<u16>,
) -> Vec<(u16, String, &ValidationError)> {
    errors
        .errors()
        .iter()
        .flat_map(|(field, err)| {
            let indent = indent.unwrap_or(0);
            let actual_path = path
                .as_ref()
                .map(|path| [path.as_str(), field].join("."))
                .unwrap_or_else(|| field.to_string());
            match err {
                ValidationErrorsKind::Field(field_errors) => field_errors
                    .iter()
                    .map(|error| (indent, actual_path.clone(), error))
                    .collect::<Vec<_>>(),
                ValidationErrorsKind::List(list_error) => list_error
                    .iter()
                    .flat_map(|(index, errors)| {
                        let actual_path =
                            format!("{}[{}]", actual_path.as_str(), index);
                        _flatten_errors(
                            errors,
                            Some(actual_path),
                            Some(indent.saturating_add(1)),
                        )
                    })
                    .collect::<Vec<_>>(),
                ValidationErrorsKind::Struct(struct_errors) => _flatten_errors(
                    struct_errors,
                    Some(actual_path),
                    Some(indent.saturating_add(1)),
                ),
            }
        })
        .collect::<Vec<_>>()
}
