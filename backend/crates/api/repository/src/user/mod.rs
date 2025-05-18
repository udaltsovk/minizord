use std::collections::HashMap;

use entity::user::{self, User};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    User
        Err: RepositoryError
    {
        async fn find_by_email(&self, email: &str) -> Option<User>;
        async fn exists_by_email(&self, email: &str) -> bool;
        async fn find_by_username(&self, username: &str) -> Option<User>;
        async fn exists_by_username(&self, username: &str) -> bool;
        async fn count_by_role(&self) -> HashMap<String, u32>;
    }
}
