use entity::{mentors, team::TeamId, user::UserId};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> mentors -> TeamId
}
