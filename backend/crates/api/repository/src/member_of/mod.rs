use macros::{crud_repository, entity};

use crate::{specialization::SpecializationId, team::TeamId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

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

crud_repository! {
    UserId -> MemberOf -> TeamId
}

impl CreateMemberOf {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> MemberOf {
        MemberOf {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            accepted: false,
            specialization: self.specialization,
        }
    }
}
