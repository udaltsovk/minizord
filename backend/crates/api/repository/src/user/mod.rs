use entity::user::{self, User};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    User {
        find_by_email(&self, email: &str) -> Option<User>;
        exists_by_email(&self, email: &str) -> bool;
        find_by_username(&self, username: &str) -> Option<User>;
        exists_by_username(&self, username: &str) -> bool;
    }
}
