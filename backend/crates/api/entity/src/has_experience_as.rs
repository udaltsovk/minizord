use macros::relation;

use crate::{EntityId, specialization::SpecializationId, user::UserId};

#[relation]
pub struct HasExperienceAs {
    pub r#in: UserId,
    pub out: SpecializationId,

    #[field]
    #[upsert]
    pub level: u16,
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
