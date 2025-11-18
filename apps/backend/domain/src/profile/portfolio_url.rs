use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

#[derive(DomainType)]
pub struct ProfilePortfolioUrl(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("portfolio_url")
        .add_constraint(constraints::length::Min(0))
        .build()
});

impl TryFrom<String> for ProfilePortfolioUrl {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
