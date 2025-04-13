use macros::entity;

use crate::{team::TeamId, technology::TechnologyId};

entity! {
    TeamId -> Uses -> TechnologyId { }
}

impl CreateUses {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Uses {
        Uses {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
