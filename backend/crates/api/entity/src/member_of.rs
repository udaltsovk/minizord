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

impl CreateMemberOf {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> MemberOf {
        MemberOf {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
            accepted: false,
            specialization: self.specialization,
        }
    }
}
