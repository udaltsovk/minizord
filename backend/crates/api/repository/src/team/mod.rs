use entity::{
    team::{self, Team},
    tour::TourId,
    user::UserId,
};
use macros::crud_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Team
        Err: RepositoryError
    {
        async fn find_by_tour_and_name(&self, tour: TourId, name: &str) -> Option<Team>;
        async fn exists_by_tour_and_name(&self, tour: TourId, name: &str) -> bool;
        async fn find_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> Option<Team>;
        async fn exists_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> bool;
        async fn find_all_by_tour(&self, tour: TourId, limit: u64, offset: u64) -> Vec<Team>;
        async fn exists_by_tour(&self, tour: TourId) -> bool;
    }
}
