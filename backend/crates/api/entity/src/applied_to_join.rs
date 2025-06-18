use macros::relation;

use crate::{EntityId, team::TeamId, user::UserId};

#[relation]
pub struct AppliedToJoin {
    pub r#in: UserId,
    pub out: TeamId,

    #[field]
    #[create]
    #[update]
    pub application: String,
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
