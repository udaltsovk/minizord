use macros::entity;

use crate::{team::TeamId, technology::TechnologyId};

entity! {
    TeamId -> Uses -> TechnologyId { }
}

impl From<CreateUses> for Uses {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_relation: CreateUses) -> Self {
        Self {
            id: create_relation.get_id(),
            r#in: create_relation.r#in,
            out: create_relation.out,
        }
    }
}
