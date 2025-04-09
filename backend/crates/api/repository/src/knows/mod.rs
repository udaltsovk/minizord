use entity::{knows, technology::TechnologyId, user::UserId};
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    UserId -> Knows -> TechnologyId
        Err: RepositoryError
}
