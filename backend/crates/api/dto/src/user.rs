use entity::user::{User as UserEntity, UserRole as UserEntityRole};
use garde::Validate;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};
use ulid::Ulid;
use utils::validation::{RE_USERNAME, validate_password};
use utoipa::ToSchema;

///
#[derive(
    Deserialize,
    Serialize,
    Display,
    EnumString,
    IntoStaticStr,
    ToSchema,
    Clone,
    Copy,
    PartialEq,
    Debug,
)]
#[serde(rename_all = "snake_case")]
#[schema(default = "participant")]
pub enum UserRole {
    ///
    #[strum(serialize = "organizer")]
    Organizer,
    ///
    #[strum(serialize = "mentor")]
    Mentor,
    ///
    #[strum(serialize = "participant")]
    Participant,
}
impl From<UserRole> for UserEntityRole {
    fn from(role: UserRole) -> Self {
        use UserRole as Role;
        match role {
            Role::Organizer => Self::Organizer,
            Role::Mentor => Self::Mentor,
            Role::Participant => Self::Participant,
        }
    }
}
impl From<UserEntityRole> for UserRole {
    fn from(role: UserEntityRole) -> Self {
        use UserEntityRole as Role;
        match role {
            Role::Organizer => Self::Organizer,
            Role::Mentor => Self::Mentor,
            Role::Participant => Self::Participant,
        }
    }
}

#[derive(Serialize, ToSchema, Clone, PartialEq, Debug)]
///
pub struct User {
    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub id: Ulid,

    ///
    #[schema(format = IdnEmail, min_length = 6, max_length = 50)]
    pub email: String,

    ///
    #[schema(
        min_length = 3,
        max_length = 20,
        pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#
    )]
    pub username: String,

    ///
    pub role: UserRole,

    ///
    pub has_profile: bool,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct CreateUser {
    ///
    #[schema(format = IdnEmail, min_length = 6, max_length = 50)]
    #[garde(length(min = 6, max = 50), email)]
    pub email: String,

    ///
    #[schema(
        min_length = 3,
        max_length = 20,
        pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#
    )]
    #[garde(length(min = 3, max = 20), pattern(*RE_USERNAME))]
    pub username: String,

    ///
    #[schema(format = Password, min_length = 8, max_length = 100)]
    #[garde(length(min = 8, max = 100), custom(validate_password))]
    pub password: String,

    ///
    #[garde(skip)]
    pub role: UserRole,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct UserUpdate {
    ///
    #[schema(format = IdnEmail, min_length = 6, max_length = 50)]
    #[garde(length(min = 6, max = 50), email)]
    pub email: Option<String>,

    ///
    #[schema(
        min_length = 3,
        max_length = 20,
        pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#
    )]
    #[garde(length(min = 3, max = 20), pattern(*RE_USERNAME))]
    pub username: Option<String>,
}

impl From<UserEntity> for User {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id.into(),
            email: entity.email,
            username: entity.username,
            role: entity.role.into(),
            has_profile: entity.profile.is_some(),
        }
    }
}
