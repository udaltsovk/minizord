use entity::specialization::{self, Specialization};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Specialization
        Err: RepositoryError
    {
        find_by_name(&self, name: &str) -> Option<Specialization>;
        exists_by_name(&self, name: &str) -> bool;
    }
}
