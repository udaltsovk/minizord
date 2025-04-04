use std::sync::Arc;

use macros::{RepositoryId, implementation};

use super::{CreateTeam, Team, TeamId, TeamUpdate};
use crate::{
    common::adapters::surrealdb::SurrealDB, tour::TourId, user::UserId,
};

impl From<TeamId> for ulid::Ulid {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(id: TeamId) -> Self {
        Self::from_string(&id.to_string()).expect("Got invalid TeamId")
    }
}

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
                .expect("Failed to save Team object!")
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
                .query(
                    r#"
                        SELECT * FROM type::table($team_table)
                            WHERE 
                                tour = type::record($tour_id)
                                name = type::string($name)
                            LIMIT 1
                    "#
                )
                .bind(("team_table", TeamId::TABLE))
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
                .query(
                    r#"
                        SELECT * FROM type::table($team_table)
                            WHERE 
                                tour = type::record($tour_id)
                                lead = type::record($lead_id)
                            LIMIT 1
                    "#
                )
                .bind(("team_table", TeamId::TABLE))
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
                .query(
                    r#"
                        SELECT * FROM type::table($team_table)
                            WHERE 
                                tour = type::record($tour_id)
                            LIMIT $limit
                            START AT $offset
                    "#
                )
                .bind(("team_table", TeamId::TABLE))
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
