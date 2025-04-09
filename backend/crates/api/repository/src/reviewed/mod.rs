use entity::{reviewed, user::UserId};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> reviewed -> UserId
        Err: RepositoryError
}
