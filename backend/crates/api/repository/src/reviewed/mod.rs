use std::collections::HashMap;

use entity::{
    reviewed::{Reviewed, UpsertReviewed},
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = UserId,
    entity = Reviewed,
    upsert = UpsertReviewed,
    error = RepositoryError
)]
pub trait ReviewedRepository {
    async fn count_by_score(&self) -> HashMap<u16, u32>;
}
