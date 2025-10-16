use lib::domain::Id;

use crate::{
    session::{
        access_token::SessionAccessToken, refresh_token::SessionRefreshToken,
    },
    user::{User, role::UserRole},
};

pub mod access_token;
pub mod refresh_token;

pub struct Session {
    pub id: Id<Self>,
    pub user: Id<User>,
    pub role: UserRole,
}

pub struct SessionTokenPair {
    pub access_token: SessionAccessToken,
    pub refresh_token: SessionRefreshToken,
}

pub struct VerboseSession {
    pub id: Id<Session>,
    pub token_pair: SessionTokenPair,
    pub user: User,
}
