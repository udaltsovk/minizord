use macros::{entity, urd_repository};

use crate::{technology::TechnologyId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

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

urd_repository! {
    UserId -> Knows -> TechnologyId
}

impl UpsertKnows {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Knows {
        Knows {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            level: self.level,
        }
    }
}
