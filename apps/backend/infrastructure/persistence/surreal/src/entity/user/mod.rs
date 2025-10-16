use domain::{profile::Profile, user::User};
use lib::infrastructure::persistence::entity::DomainTypeFromDb as _;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{SurrealId, user::role::StoredUserRole},
    impl_table_for,
};

pub(crate) mod role;

impl_table_for!(User);

#[derive(Deserialize, Serialize)]
pub struct StoredUser {
    pub id: SurrealId<User>,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: StoredUserRole,
    pub profile_id: Option<SurrealId<Profile>>,
}

impl From<User> for StoredUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id.into(),
            email: u.email.into(),
            username: u.username.into(),
            password_hash: u.password_hash,
            role: u.role.into(),
            profile_id: u.profile_id.map(SurrealId::from),
        }
    }
}

impl From<StoredUser> for User {
    fn from(u: StoredUser) -> Self {
        Self {
            id: u.id.into(),
            email: u.email.into_domain(),
            username: u.username.into_domain(),
            password_hash: u.password_hash,
            role: u.role.into(),
            profile_id: u.profile_id.map(SurrealId::into),
        }
    }
}
