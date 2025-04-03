use macros::{entity, urd_repository};

use crate::{specialization::SpecializationId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    UserId -> HasExperienceAs -> SpecializationId {
        fields {
            level: u16,
        },
        upsert {
            level: u16,
        }
    }
}

urd_repository! {
    UserId -> HasExperienceAs -> SpecializationId
}

impl UpsertHasExperienceAs {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> HasExperienceAs {
        HasExperienceAs {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            level: self.level,
        }
    }
}
