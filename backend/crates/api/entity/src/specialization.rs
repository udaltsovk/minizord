use macros::entity;
use ulid::Ulid;

entity! {
    Specialization {
        id: Ulid,
        fields {
            name: String,
        },
        create {
            name: String,
        },
        update {
            name: String,
        }
    }
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
