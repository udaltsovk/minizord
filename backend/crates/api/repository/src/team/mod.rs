use macros::{RepositoryId, crud_repository};
use ulid::Ulid;

use crate::user::UserId;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    Team {
        id: Ulid,
        fields {
            name: String,
            lead: Ulid,
            tour: Ulid,
        },
        create {
            name: String,
            lead: Ulid,
            tour: Ulid,
        },
        update {
            name: String,
        }
    } {
        find_by_tour_and_name(&self, tour: Ulid, name: &str) -> Option<Team>;
        exists_by_tour_and_name(&self, tour: Ulid, name: &str) -> bool;
        find_by_tour_and_lead(&self, tour: Ulid, lead: UserId) -> Option<Team>;
        exists_by_tour_and_lead(&self, tour: Ulid, lead: UserId) -> bool;
        find_all_by_tour(&self, tour: Ulid, limit: u64, offset: u64) -> Vec<Team>;
        exists_by_tour(&self, tour: Ulid) -> bool;
    }
}

impl CreateTeam {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Team {
        Team {
            id: TeamId::from(Ulid::new()),
            name: self.name.clone(),
            lead: self.lead,
            tour: self.tour,
        }
    }
}
