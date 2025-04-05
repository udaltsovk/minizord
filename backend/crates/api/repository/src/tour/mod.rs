use entity::tour::{self, Tour};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Tour {
        find_by_name(&self, name: &str) -> Option<Tour>;
        exists_by_name(&self, name: &str) -> bool;
    }
}
