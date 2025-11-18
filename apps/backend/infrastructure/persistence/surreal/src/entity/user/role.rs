use domain::user::role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StoredUserRole {
    Admin,
    Organizer,
    Mentor,
    Participant,
}

impl From<StoredUserRole> for UserRole {
    fn from(role: StoredUserRole) -> Self {
        use StoredUserRole as R;
        match role {
            R::Admin => Self::Admin,
            R::Organizer => Self::Organizer,
            R::Mentor => Self::Mentor,
            R::Participant => Self::Participant,
        }
    }
}

impl From<UserRole> for StoredUserRole {
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
