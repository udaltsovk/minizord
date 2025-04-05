use entity::{
    team::{self, Team},
    tour::TourId,
    user::UserId,
};
use macros::crud_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Team {
        find_by_tour_and_name(&self, tour: TourId, name: &str) -> Option<Team>;
        exists_by_tour_and_name(&self, tour: TourId, name: &str) -> bool;
        find_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> Option<Team>;
        exists_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> bool;
        find_all_by_tour(&self, tour: TourId, limit: u64, offset: u64) -> Vec<Team>;
        exists_by_tour(&self, tour: TourId) -> bool;
    }
}
