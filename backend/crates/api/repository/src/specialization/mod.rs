use macros::{RepositoryId, crud_repository, entity};
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

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

crud_repository! {
    Specialization {
        find_by_name(&self, name: &str) -> Option<Specialization>;
        exists_by_name(&self, name: &str) -> bool;
    }
}

impl CreateSpecialization {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Specialization {
        Specialization {
            id: SpecializationId::from(Ulid::new()),
            name: self.name.clone(),
        }
    }
}
