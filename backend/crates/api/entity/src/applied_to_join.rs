use macros::entity;

use crate::{team::TeamId, user::UserId};

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

impl From<CreateAppliedToJoin> for AppliedToJoin {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_relation: CreateAppliedToJoin) -> Self {
        AppliedToJoin {
            id: create_relation.get_id(),
            r#in: create_relation.r#in,
            out: create_relation.out,
            application: create_relation.application,
        }
    }
}
