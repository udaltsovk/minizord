use std::collections::HashMap;

use entity::{reviewed, user::UserId};
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    UserId -> reviewed -> UserId
        Err: RepositoryError
    {
        async fn count_by_score(&self) -> HashMap<u16, u32>;
    }
}
