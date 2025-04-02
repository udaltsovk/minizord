use macros::{RepositoryId, crud_repository};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use ulid::Ulid;

use crate::profile::ProfileId;

#[cfg(feature = "surrealdb")]
pub mod surreal;

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

crud_repository! {
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
    } {
        find_by_email(&self, email: &str) -> Option<User>;
        exists_by_email(&self, email: &str) -> bool;
        find_by_username(&self, username: &str) -> Option<User>;
        exists_by_username(&self, username: &str) -> bool;
    }
}

impl CreateUser {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> User {
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
