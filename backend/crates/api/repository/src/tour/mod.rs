use entity::tour::{self, Tour};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Tour
        Err: RepositoryError
    {
        async fn find_by_name(&self, name: &str) -> Option<Tour>;
        async fn exists_by_name(&self, name: &str) -> bool;
    }
}
