use utils::validation::validation_errors_to_string;
use validator::Validate;

use super::HandlerError;

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate(obj: impl Validate) -> Result<(), HandlerError> {
    obj.validate().map_err(|errors| {
        HandlerError::InvalidInput(validation_errors_to_string(errors, None))
    })
}
