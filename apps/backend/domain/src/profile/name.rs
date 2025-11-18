use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};
use regex::Regex;

use crate::validation::regex::NAME_REGEX;

#[derive(DomainType)]
pub struct ProfileName(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("name")
        .add_constraint(constraints::length::Min(2))
        .add_constraint(constraints::length::Max(24))
        .add_constraint(constraints::Matches(
            Regex::new(NAME_REGEX).expect("a valid regex"),
        ))
        .build()
});

impl TryFrom<String> for ProfileName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
