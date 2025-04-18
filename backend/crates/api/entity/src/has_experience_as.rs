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

impl From<UpsertHasExperienceAs> for HasExperienceAs {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(upsert_relation: UpsertHasExperienceAs) -> Self {
        Self {
            id: upsert_relation.get_id(),
            r#in: upsert_relation.r#in,
            out: upsert_relation.out,
            level: upsert_relation.level,
        }
    }
}
