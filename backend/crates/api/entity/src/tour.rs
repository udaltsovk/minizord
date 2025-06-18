use chrono::{DateTime, Utc};
use macros::entity;
use ulid::Ulid;

use crate::{EntityId, specialization::SpecializationId};

#[entity]
pub struct Tour {
    pub id: Ulid,

    #[field]
    #[create]
    #[update]
    pub name: String,

    #[field]
    #[create]
    #[update]
    pub starts_at: DateTime<Utc>,

    #[field]
    #[create]
    #[update]
    pub ends_at: DateTime<Utc>,

    #[field]
    #[create]
    pub max_members: u16,

    #[field]
    #[create]
    pub required_specializations: Vec<SpecializationId>,
}

impl From<CreateTour> for Tour {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_entity: CreateTour) -> Self {
        Self {
            id: TourId::from(Ulid::new()),
            name: create_entity.name,
            starts_at: create_entity.starts_at,
            ends_at: create_entity.ends_at,
            max_members: create_entity.max_members,
            required_specializations: create_entity.required_specializations,
        }
    }
}
