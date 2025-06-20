use macros::relation;

use crate::{
    EntityId, specialization::SpecializationId, team::TeamId, user::UserId,
};

#[relation]
pub struct MemberOf {
    pub r#in: UserId,
    pub out: TeamId,

    #[field]
    #[update]
    pub accepted: bool,

    #[field]
    #[create]
    #[update]
    pub specialization: SpecializationId,
}

impl From<CreateMemberOf> for MemberOf {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_relation: CreateMemberOf) -> Self {
        Self {
            id: create_relation.get_id(),
            r#in: create_relation.r#in,
            out: create_relation.out,
            accepted: false,
            specialization: create_relation.specialization,
        }
    }
}
