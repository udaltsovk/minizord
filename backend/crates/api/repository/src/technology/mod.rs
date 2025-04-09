use entity::technology::{self, Technology};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Technology
        Err: RepositoryError
    {
        find_by_name(&self, name: &str) -> Option<Technology>;
        exists_by_name(&self, name: &str) -> bool;
    }
}
