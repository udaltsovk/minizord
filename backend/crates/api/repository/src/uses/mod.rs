use macros::crud_repository;

use crate::{team::TeamId, technology::TechnologyId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    TeamId -> Uses -> TechnologyId { }
}

impl CreateUses {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Uses {
        Uses {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
