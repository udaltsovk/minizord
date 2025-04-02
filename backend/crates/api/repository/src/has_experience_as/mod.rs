use macros::crud_repository;

use crate::{specialization::SpecializationId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> HasExperienceAs -> SpecializationId { }
}

impl CreateHasExperienceAs {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> HasExperienceAs {
        HasExperienceAs {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
