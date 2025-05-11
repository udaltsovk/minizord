use entity::technology::{self, Technology};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Technology
        Err: RepositoryError
    {
        async fn find_by_name(&self, name: &str) -> Option<Technology>;
        async fn exists_by_name(&self, name: &str) -> bool;
    }
}
