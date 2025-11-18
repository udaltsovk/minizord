use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

use crate::validation::constraints::SecurePassword;

#[derive(DomainType)]
pub struct UserPassword(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("password")
        .add_constraint(constraints::length::Min(8))
        .add_constraint(SecurePassword)
        .build()
});

impl TryFrom<String> for UserPassword {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
