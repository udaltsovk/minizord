use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

#[derive(DomainType)]
pub struct ProfileBio(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("bio")
        .add_constraint(constraints::length::Min(0))
        .add_constraint(constraints::length::Max(4096))
        .build()
});

impl TryFrom<String> for ProfileBio {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
