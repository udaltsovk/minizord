use entity::{participated_in, tour::TourId, user::UserId};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> participated_in -> TourId
        Err: RepositoryError
}
