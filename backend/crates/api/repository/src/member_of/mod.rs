use entity::{member_of, team::TeamId, user::UserId};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> member_of -> TeamId
}
