use entity::{mentors, team::TeamId, user::UserId};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> mentors -> TeamId
        Err: RepositoryError
}
