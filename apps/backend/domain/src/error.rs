use lib::domain::validation::error::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error(transparent)]
    Validation(ValidationErrors),
}
