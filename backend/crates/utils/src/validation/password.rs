use passwords::{analyzer::analyze, scorer::score};
use validator::ValidationError;

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if score(&analyze(password)) < 57.0 {
        Err(ValidationError::new("Password is too weak"))
    } else {
        Ok(())
    }
}
