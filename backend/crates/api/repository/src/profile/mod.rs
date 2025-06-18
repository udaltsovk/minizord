use std::collections::HashMap;

use entity::profile::{Profile, ProfileId, UpsertProfile};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = Profile,
    entity_id = ProfileId,
    upsert = UpsertProfile,
    error = RepositoryError
)]
pub trait ProfileRepository {
    async fn count_by_city(&self) -> HashMap<String, u32>;
}
