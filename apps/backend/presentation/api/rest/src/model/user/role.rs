use domain::user::role::UserRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

///
#[derive(Deserialize, Serialize, ToSchema, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
#[schema(default = "participant")]
pub enum JsonUserRole {
    ///
    Admin,
    ///
    Organizer,
    ///
    Mentor,
    ///
    Participant,
}

impl From<JsonUserRole> for UserRole {
    fn from(role: JsonUserRole) -> Self {
        use JsonUserRole as R;
        match role {
            R::Admin => Self::Admin,
            R::Organizer => Self::Organizer,
            R::Mentor => Self::Mentor,
            R::Participant => Self::Participant,
        }
    }
}

impl From<UserRole> for JsonUserRole {
    fn from(role: UserRole) -> Self {
        use UserRole as R;
        match role {
            R::Admin => Self::Admin,
            R::Organizer => Self::Organizer,
            R::Mentor => Self::Mentor,
            R::Participant => Self::Participant,
        }
    }
}
