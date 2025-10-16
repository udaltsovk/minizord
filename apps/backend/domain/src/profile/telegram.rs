use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};
use regex::Regex;

#[derive(DomainType)]
pub struct ProfileTelegram(String);

pub const TELEGRAM_REGEX: &str = r#"^[A-Za-z\d_]{5,32}$"#;

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("telegram")
        .add_constraint(constraints::length::Min(5))
        .add_constraint(constraints::length::Max(32))
        .add_constraint(constraints::Matches(
            Regex::new(TELEGRAM_REGEX).expect("a valid regex"),
        ))
        .build()
});

impl TryFrom<String> for ProfileTelegram {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
