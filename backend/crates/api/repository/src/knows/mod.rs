use macros::repository;

use crate::{technology::TechnologyId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    UserId -> Knows -> TechnologyId { }
}

impl CreateKnows {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Knows {
        Knows {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
