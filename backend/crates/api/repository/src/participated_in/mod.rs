use entity::{participated_in, tour::TourId, user::UserId};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> participated_in -> TourId
}
