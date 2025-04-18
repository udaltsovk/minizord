use macros::entity;

use crate::{specialization::SpecializationId, team::TeamId, user::UserId};

entity! {
    UserId -> MemberOf -> TeamId {
        fields {
            accepted: bool,
            specialization: SpecializationId,
        },
        create {
            specialization: SpecializationId,
        },
        update {
            accepted: bool,
            specialization: SpecializationId,
        }
    }
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
