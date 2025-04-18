use chrono::{DateTime, Utc};
use macros::entity;
use ulid::Ulid;

use crate::specialization::SpecializationId;

entity! {
    Tour {
        id: Ulid,
        fields {
            name: String,
            starts_at: DateTime<Utc>,
            ends_at: DateTime<Utc>,
            max_members: u16,
            required_specializations: Vec<SpecializationId>,
        },
        create {
            name: String,
            starts_at: DateTime<Utc>,
            ends_at: DateTime<Utc>,
            max_members: u16,
            required_specializations: Vec<SpecializationId>,
        },
        update {
            name: String,
            starts_at: DateTime<Utc>,
            ends_at: DateTime<Utc>,
            required_specializations: Vec<SpecializationId>,
        }
    }
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
