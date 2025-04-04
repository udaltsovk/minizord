use macros::{RepositoryId, crud_repository, entity};
use ulid::Ulid;

use crate::{tour::TourId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    Team {
        id: Ulid,
        fields {
            name: String,
            lead: UserId,
            tour: TourId,
        },
        create {
            name: String,
            lead: UserId,
            tour: TourId,
        },
        update {
            name: String,
        }
    }
}

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
