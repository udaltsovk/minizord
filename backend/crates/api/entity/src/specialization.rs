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

impl CreateSpecialization {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Specialization {
        Specialization {
            id: SpecializationId::from(Ulid::new()),
            name: self.name.clone(),
        }
    }
}
