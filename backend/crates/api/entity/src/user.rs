use macros::entity;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use ulid::Ulid;

use crate::profile::ProfileId;

#[derive(Deserialize, Serialize, Display, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[strum(serialize = "organizator")]
    Organizator,
    #[strum(serialize = "mentor")]
    Mentor,
    #[strum(serialize = "participant")]
    Participant,
}

entity! {
    User {
        id: Ulid,
        fields {
            email: String,
            password_hash: String,
            username: String,
            role: UserRole,
            profile: Option<ProfileId>,
        },
        create {
            email: String,
            password_hash: String,
            username: String,
            role: UserRole,
        },
        update {
            email: String,
            password_hash: String,
            username: String,
            role: UserRole,
            profile: Option<ProfileId>,
        }
    }
}

impl CreateUser {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> User {
        User {
            id: UserId::from(Ulid::new()),
            email: self.email,
            password_hash: self.password_hash,
            username: self.username,
            role: self.role,
            profile: None,
        }
    }
}
