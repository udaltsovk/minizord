use entity::user::{User as UserEntity, UserRole as UserEntityRole};
use macros::dto;
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
        fields {
            ///
            #[schema(format = Ulid)]
            #[garde(skip)]
            id: Ulid,

            ///
            #[schema(format = Email, min_length = 6, max_length = 50)]
            #[garde(skip)]
            email: String,

            ///
            #[schema(min_length = 3, max_length = 20, pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#)]
            #[garde(skip)]
            username: String,

            ///
            #[garde(skip)]
            role: UserRole,
        },
        create
        ///
        {
            ///
            #[schema(format = Email, min_length = 6, max_length = 50)]
            #[garde(length(min = 6, max = 50), email)]
            email: String,

            ///
            #[schema(min_length = 3, max_length = 20, pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#)]
            #[garde(length(min = 3, max = 20), pattern(*RE_USERNAME))]
            username: String,

            ///
            #[schema(format = Password, min_length = 8, max_length = 100)]
            #[garde(length(min = 8, max = 100), custom(validate_password))]
            password: String,

            ///
            #[garde(skip)]
            role: UserRole,
        },
        update
        ///
        {
            ///
            #[schema(format = Email, min_length = 6, max_length = 50)]
            #[garde(length(min = 3, max = 20), pattern(*RE_USERNAME))]
            email: String,

            ///
            #[schema(min_length = 3, max_length = 20, pattern = r#"^[a-zA-Z0-9._-]{3,20}$"#)]
            #[garde(length(min = 8, max = 100), inner(custom(validate_password)))]
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
