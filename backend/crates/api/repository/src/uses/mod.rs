use entity::{team::TeamId, technology::TechnologyId, uses};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    TeamId -> uses -> TechnologyId
        Err: RepositoryError
}
