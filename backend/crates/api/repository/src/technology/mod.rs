use macros::{RepositoryId, crud_repository, entity};
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    Technology {
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
    Technology {
        find_by_name(&self, name: &str) -> Option<Technology>;
        exists_by_name(&self, name: &str) -> bool;
    }
}

impl CreateTechnology {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Technology {
        Technology {
            id: TechnologyId::from(Ulid::new()),
            name: self.name.clone(),
        }
    }
}
