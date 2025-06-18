use entity::technology::{
    CreateTechnology, Technology, TechnologyId, TechnologyUpdate,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = Technology,
    entity_id = TechnologyId,
    create = CreateTechnology,
    update = TechnologyUpdate,
    error = RepositoryError
)]
pub trait TechnologyRepository {
    async fn find_by_name(&self, name: &str) -> Option<Technology>;
    async fn exists_by_name(&self, name: &str) -> bool;
}
