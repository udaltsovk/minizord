use macros::entity;
use ulid::Ulid;

use crate::{EntityId, tour::TourId, user::UserId};

#[entity]
pub struct Team {
    pub id: Ulid,

    #[field]
    #[create]
    #[update]
    pub name: String,

    #[field]
    #[create]
    pub lead: UserId,

    #[field]
    #[create]
    pub tour: TourId,
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
