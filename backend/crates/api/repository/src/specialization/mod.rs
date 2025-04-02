use macros::{RepositoryId, crud_repository};
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
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
    } {
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
