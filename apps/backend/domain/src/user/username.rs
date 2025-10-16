use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};
use regex::Regex;

#[derive(DomainType)]
pub struct Username(String);

const USERNAME_REGEX: &str = r#"^[a-zA-Z0-9._-]{3,20}$"#;

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("username")
        .add_constraint(constraints::length::Min(3))
        .add_constraint(constraints::length::Max(20))
        .add_constraint(constraints::Matches(
            Regex::new(USERNAME_REGEX).expect("a valid regex"),
        ))
        .build()
});

impl TryFrom<String> for Username {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
