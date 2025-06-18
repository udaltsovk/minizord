use std::collections::HashMap;

use entity::user::{CreateUser, User, UserId, UserUpdate};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = User,
    entity_id = UserId,
    create = CreateUser,
    update = UserUpdate,
    error = RepositoryError
)]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn exists_by_email(&self, email: &str) -> bool;
    async fn find_by_username(&self, username: &str) -> Option<User>;
    async fn exists_by_username(&self, username: &str) -> bool;
    async fn count_by_role(&self) -> HashMap<String, u32>;
}
