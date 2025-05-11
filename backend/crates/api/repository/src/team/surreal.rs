use std::sync::Arc;

use entity::{
    team::{CreateTeam, Team, TeamId, TeamUpdate},
    tour::TourId,
    user::UserId,
};
use macros::{EntityId, implementation, surql_query};
use utils::adapters::SurrealDB;

use super::{TeamRepository, TeamRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TeamRepository {
        db: Arc<SurrealDB>
    } as SurrealTeamRepository {
        async fn save(&self, new: CreateTeam) -> Team {
            let entity: Team = new.into();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        async fn find_by_id(&self, id: TeamId) -> Option<Team> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        async fn exists_by_id(&self, id: TeamId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        async fn find_by_tour_and_name(&self, tour: TourId, name: &str) -> Option<Team> {
            self.db.0
                .query(surql_query!("table/team/find_by_tour_and_name"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        async fn exists_by_tour_and_name(&self, tour: TourId, name: &str) -> bool {
            self.find_by_tour_and_name(tour, name).await?.is_some()
        }

        async fn find_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> Option<Team> {
            self.db.0
                .query(surql_query!("table/team/find_by_tour_and_lead"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("lead_id", lead))
                .await?
                .take(0)?
        }

        async fn exists_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> bool {
            self.find_by_tour_and_lead(tour, lead).await?.is_some()
        }

        async fn find_all_by_tour(&self, tour: TourId, limit: u64, offset: u64) -> Vec<Team> {
            self.db.0
                .query(surql_query!("table/team/find_all_by_tour"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        async fn exists_by_tour(&self, tour: TourId) -> bool {
            !self.find_all_by_tour(tour, 1, 0).await?.is_empty()
        }

        async fn update_by_id(&self, id: TeamId, update: TeamUpdate) -> Option<Team> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        async fn delete_by_id(&self, id: TeamId) -> Option<Team> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
