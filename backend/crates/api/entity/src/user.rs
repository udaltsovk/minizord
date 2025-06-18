use macros::entity;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use ulid::Ulid;

use crate::{EntityId, profile::ProfileId};

#[derive(Deserialize, Serialize, Display, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[strum(serialize = "organizer")]
    Organizer,
    #[strum(serialize = "mentor")]
    Mentor,
    #[strum(serialize = "participant")]
    Participant,
}

#[entity]
pub struct User {
    pub id: Ulid,

    #[field]
    #[create]
    #[update]
    pub email: String,

    #[field]
    #[create]
    #[update]
    pub password_hash: String,

    #[field]
    #[create]
    #[update]
    pub username: String,

    #[field]
    #[create]
    #[update]
    pub role: UserRole,

    #[field]
    #[update]
    pub profile: Option<ProfileId>,
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
