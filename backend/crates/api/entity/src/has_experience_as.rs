use macros::entity;

use crate::{specialization::SpecializationId, user::UserId};

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

impl UpsertHasExperienceAs {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> HasExperienceAs {
        HasExperienceAs {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
            level: self.level,
        }
    }
}
