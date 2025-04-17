use macros::entity;
use ulid::Ulid;

use crate::{tour::TourId, user::UserId};

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

impl CreateTeam {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Team {
        Team {
            id: TeamId::from(Ulid::new()),
            name: self.name,
            lead: self.lead,
            tour: self.tour,
        }
    }
}
