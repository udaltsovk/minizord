use entity::{applied_to_join, team::TeamId, user::UserId};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> applied_to_join -> TeamId
        Err: RepositoryError
}
