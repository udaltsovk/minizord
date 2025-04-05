use entity::{reviewed, user::UserId};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> reviewed -> UserId
}
