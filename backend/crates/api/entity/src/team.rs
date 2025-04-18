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

impl From<CreateTeam> for Team {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_entity: CreateTeam) -> Self {
        Self {
            id: TeamId::from(Ulid::new()),
            name: create_entity.name,
            lead: create_entity.lead,
            tour: create_entity.tour,
        }
    }
}
