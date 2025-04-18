use macros::entity;

use crate::{team::TeamId, user::UserId};

entity! {
    UserId -> Mentors -> TeamId { }
}

impl From<CreateMentors> for Mentors {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_relation: CreateMentors) -> Self {
        Self {
            id: create_relation.get_id(),
            r#in: create_relation.r#in,
            out: create_relation.out,
        }
    }
}
