use entity::tour::{CreateTour, Tour, TourId, TourUpdate};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = Tour,
    entity_id = TourId,
    create = CreateTour,
    update = TourUpdate,
    error = RepositoryError
)]
pub trait TourRepository {
    async fn find_by_name(&self, name: &str) -> Option<Tour>;
    async fn exists_by_name(&self, name: &str) -> bool;
}
