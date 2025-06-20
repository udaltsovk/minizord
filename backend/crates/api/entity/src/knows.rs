use macros::relation;

use crate::{EntityId, technology::TechnologyId, user::UserId};

#[relation]
pub struct Knows {
    pub r#in: UserId,
    pub out: TechnologyId,

    #[field]
    #[upsert]
    pub level: u16,
}

impl From<UpsertKnows> for Knows {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(upsert_relation: UpsertKnows) -> Self {
        Self {
            id: upsert_relation.get_id(),
            r#in: upsert_relation.r#in,
            out: upsert_relation.out,
            level: upsert_relation.level,
        }
    }
}
