use std::sync::Arc;

use entity::{
    team::{CreateTeam, Team, TeamId, TeamUpdate},
    tour::TourId,
    user::UserId,
};
use macros::{EntityId, implementation};
use utils::adapters::SurrealDB;

use super::{TeamRepository, TeamRepositoryResult};
use crate::common::RepositoryError;

implementation! {
    TeamRepository {
        db: Arc<SurrealDB>
    } as Surreal {
        save(&self, new: CreateTeam) -> Team {
            let entity = new.into_entity();
            self.db.0
                .create(entity.id.record_id())
                .content(entity)
                .await?
                .ok_or(RepositoryError::FailedToSaveObject)?
        }

        find_by_id(&self, id: TeamId) -> Option<Team> {
            self.db.0
                .select(id.record_id())
                .await?
        }

        exists_by_id(&self, id: TeamId) -> bool {
            self.find_by_id(id).await?.is_some()
        }

        find_by_tour_and_name(&self, tour: TourId, name: &str) -> Option<Team> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/table/team/find_by_tour_and_name.surql"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("name", name.to_string()))
                .await?
                .take(0)?
        }

        exists_by_tour_and_name(&self, tour: TourId, name: &str) -> bool {
            self.find_by_tour_and_name(tour, name).await?.is_some()
        }

        find_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> Option<Team> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/table/team/find_by_tour_and_lead.surql"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("lead_id", lead))
                .await?
                .take(0)?
        }

        exists_by_tour_and_lead(&self, tour: TourId, lead: UserId) -> bool {
            self.find_by_tour_and_lead(tour, lead).await?.is_some()
        }

        find_all_by_tour(&self, tour: TourId, limit: u64, offset: u64) -> Vec<Team> {
            self.db.0
                .query(include_str!("../../db/surreal/queries/table/team/find_all_by_tour.surql"))
                .bind(("table", TeamId::TABLE))
                .bind(("tour_id", tour))
                .bind(("limit", limit))
                .bind(("offset", offset))
                .await?
                .take(0)?
        }

        exists_by_tour(&self, tour: TourId) -> bool {
            !self.find_all_by_tour(tour, 1, 0).await?.is_empty()
        }

        update_by_id(&self, id: TeamId, update: TeamUpdate) -> Option<Team> {
            self.db.0
                .update(id.record_id())
                .merge(update)
                .await?
        }

        delete_by_id(&self, id: TeamId) -> Option<Team> {
            self.db.0
                .delete(id.record_id())
                .await?
        }
    }
}
