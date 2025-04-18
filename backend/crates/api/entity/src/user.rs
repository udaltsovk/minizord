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

impl From<CreateUser> for User {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_entity: CreateUser) -> Self {
        Self {
            id: UserId::from(Ulid::new()),
            email: create_entity.email,
            password_hash: create_entity.password_hash,
            username: create_entity.username,
            role: create_entity.role,
            profile: None,
        }
    }
}
