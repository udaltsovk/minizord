use macros::entity;
use ulid::Ulid;

use crate::EntityId;

#[entity]
pub struct Specialization {
    pub id: Ulid,

    #[field]
    #[create]
    #[update]
    pub name: String,
}

impl From<CreateSpecialization> for Specialization {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_entity: CreateSpecialization) -> Self {
        Self {
            id: SpecializationId::from(Ulid::new()),
            name: create_entity.name,
        }
    }
}
