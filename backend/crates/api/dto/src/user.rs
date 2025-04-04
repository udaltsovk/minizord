use macros::dto;
use repository::user::{User as UserEntity, UserRole as UserEntityRole};
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
    #[strum(serialize = "organizator")]
    Organizator,
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
            Role::Organizator => Self::Organizator,
            Role::Mentor => Self::Mentor,
            Role::Participant => Self::Participant,
        }
    }
}
impl From<UserEntityRole> for UserRole {
    fn from(role: UserEntityRole) -> Self {
        use UserEntityRole as Role;
        match role {
            Role::Organizator => Self::Organizator,
            Role::Mentor => Self::Mentor,
            Role::Participant => Self::Participant,
        }
    }
}

dto! {
    ///
    User {
        ///
        #[schema(format = Ulid)]
        id: Ulid,
        fields {
            ///
            #[validate(length(min = 6, max = 50), email)]
            #[schema(format = Email, min_length = 6, max_length = 50)]
            email: String,

            ///
            #[validate(length(min = 3, max = 20), regex(path = *RE_USERNAME))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,

            ///
            role: UserRole,
        },
        create
        ///
        {
            ///
            #[validate(length(min = 6, max = 50), email)]
            #[schema(format = Email, min_length = 6, max_length = 50)]
            email: String,

            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,

            ///
            #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
            #[schema(format = Password, min_length = 8, max_length = 100)]
            password: String,

            ///
            role: UserRole,
        },
        update
        ///
        {
            ///
            #[validate(length(min = 6, max = 50), email)]
            #[schema(format = Email, min_length = 6, max_length = 50)]
            email: String,

            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,
        }
    }
}

impl From<UserEntity> for User {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id.into(),
            email: entity.email,
            username: entity.username,
            role: entity.role.into(),
        }
    }
}
