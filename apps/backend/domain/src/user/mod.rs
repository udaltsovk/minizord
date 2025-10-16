use lib::domain::Id;

use crate::{
    profile::Profile,
    user::{
        email::UserEmail, password::UserPassword, role::UserRole,
        username::Username,
    },
};

pub mod email;
pub mod password;
pub mod role;
pub mod username;

pub struct User {
    pub id: Id<User>,
    pub email: UserEmail,
    pub username: Username,
    pub password_hash: String,
    pub role: UserRole,
    pub profile_id: Option<Id<Profile>>,
}

pub struct RegisterUser {
    pub email: UserEmail,
    pub username: Username,
    pub password: UserPassword,
    pub role: UserRole,
}
