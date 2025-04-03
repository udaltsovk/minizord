use chrono::{DateTime, Utc};
use macros::{RepositoryId, crud_repository, entity};
use ulid::Ulid;

use crate::specialization::SpecializationId;

#[cfg(feature = "surrealdb")]
pub mod surreal;

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

crud_repository! {
    Tour {
        find_by_name(&self, name: &str) -> Option<Tour>;
        exists_by_name(&self, name: &str) -> bool;
    }
}

impl CreateTour {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Tour {
        Tour {
            id: TourId::from(Ulid::new()),
            name: self.name.clone(),
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            max_members: self.max_members,
            required_specializations: self.required_specializations,
        }
    }
}
