use entity::{reviewed, user::UserId};
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    UserId -> reviewed -> UserId
        Err: RepositoryError
}
