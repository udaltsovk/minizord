use entity::{team::TeamId, technology::TechnologyId, uses};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    TeamId -> uses -> TechnologyId
}
