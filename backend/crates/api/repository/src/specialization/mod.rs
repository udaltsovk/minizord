use entity::specialization::{
    CreateSpecialization, Specialization, SpecializationId,
    SpecializationUpdate,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = Specialization,
    entity_id = SpecializationId,
    create = CreateSpecialization,
    update = SpecializationUpdate,
    error = RepositoryError
)]
pub trait SpecializationRepository {
    async fn find_by_name(&self, name: &str) -> Option<Specialization>;
    async fn exists_by_name(&self, name: &str) -> bool;
}
