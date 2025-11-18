use domain::user::{
    RegisterUser, User, password::UserPassword, username::Username,
};
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    presentation::api::rest::model::ParseableJson,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::user::role::JsonUserRole;

mod role;

///
#[derive(Serialize, ToSchema)]
pub struct JsonUser {
    ///
    pub id: Uuid,

    ///
    pub email: String,

    ///
    pub username: String,

    ///
    pub role: JsonUserRole,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<Uuid>,
}

impl From<User> for JsonUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id.value,
            email: u.email.into(),
            username: u.username.into(),
            role: u.role.into(),
            profile_id: u.profile_id.map(Uuid::from),
        }
    }
}

///
#[derive(Deserialize, ToSchema)]
pub struct RegisterJsonUser {
    ///
    pub email: String,

    ///
    pub username: String,

    ///
    pub password: String,

    ///
    pub role: JsonUserRole,
}

impl ParseableJson<RegisterUser> for RegisterJsonUser {
    fn parse(self) -> Result<RegisterUser, ValidationErrors> {
        let (errors, (email, username, password)) =
            into_validators!(self.email, self.username, self.password);

        errors.into_result(|ok| RegisterUser {
            email: email.validated(ok),
            username: username.validated(ok),
            password: password.validated(ok),
            role: self.role.into(),
        })
    }
}

///
#[derive(Deserialize, ToSchema)]
pub struct LoginJsonUser {
    ///
    pub username: String,

    ///
    pub password: String,
}

impl ParseableJson<(Username, UserPassword)> for LoginJsonUser {
    fn parse(self) -> Result<(Username, UserPassword), ValidationErrors> {
        let (errors, (username, password)) =
            into_validators!(self.username, self.password);

        errors
            .into_result(|ok| (username.validated(ok), password.validated(ok)))
    }
}
