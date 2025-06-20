use entity::{
    team::{CreateTeam, Team, TeamId, TeamUpdate},
    tour::TourId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    entity = Team,
    entity_id = TeamId,
    create = CreateTeam,
    update = TeamUpdate,
    error = RepositoryError
)]
pub trait TeamRepository {
    async fn find_by_tour_and_name(
        &self,
        tour: TourId,
        name: &str,
    ) -> Option<Team>;
    async fn exists_by_tour_and_name(&self, tour: TourId, name: &str) -> bool;
    async fn find_by_tour_and_lead(
        &self,
        tour: TourId,
        lead: UserId,
    ) -> Option<Team>;
    async fn exists_by_tour_and_lead(&self, tour: TourId, lead: UserId)
    -> bool;
    async fn find_all_by_tour(
        &self,
        tour: TourId,
        limit: u64,
        offset: u64,
    ) -> Vec<Team>;
    async fn exists_by_tour(&self, tour: TourId) -> bool;
}
