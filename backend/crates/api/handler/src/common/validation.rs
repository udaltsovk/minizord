use super::HandlerError;
use utils::validation::validation_errors_to_string;
use validator::Validate;

#[tracing::instrument(skip_all)]
pub fn validate(obj: impl Validate) -> Result<(), HandlerError> {
    obj.validate()
        .map_err(|errors| HandlerError::InvalidInput(validation_errors_to_string(errors, None)))
}
