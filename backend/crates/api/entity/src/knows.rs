use macros::entity;

use crate::{technology::TechnologyId, user::UserId};

entity! {
    UserId -> Knows -> TechnologyId {
        fields {
            level: u16,
        },
        upsert {
            level: u16,
        }
    }
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
