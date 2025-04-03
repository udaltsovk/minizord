use macros::{crud_repository, entity};

use crate::{team::TeamId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
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

crud_repository! {
    UserId -> AppliedToJoin -> TeamId
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
