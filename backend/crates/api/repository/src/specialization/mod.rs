use entity::specialization::{self, Specialization};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Specialization
        Err: RepositoryError
    {
        async fn find_by_name(&self, name: &str) -> Option<Specialization>;
        async fn exists_by_name(&self, name: &str) -> bool;
    }
}
