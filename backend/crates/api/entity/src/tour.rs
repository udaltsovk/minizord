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

impl CreateTour {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Tour {
        Tour {
            id: TourId::from(Ulid::new()),
            name: self.name,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            max_members: self.max_members,
            required_specializations: self.required_specializations,
        }
    }
}
