use macros::repository;

use crate::{team::TeamId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    UserId -> AppliedToJoin -> TeamId {
        fields {
            application: String,
        },
        create {
            application: String,
        },
        update {
            application: String,
        }
    }
}

impl CreateAppliedToJoin {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> AppliedToJoin {
        AppliedToJoin {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            application: self.application,
        }
    }
}
