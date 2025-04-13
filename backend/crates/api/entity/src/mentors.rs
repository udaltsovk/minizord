use macros::entity;

use crate::{team::TeamId, user::UserId};

entity! {
    UserId -> Mentors -> TeamId { }
}

impl CreateMentors {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Mentors {
        Mentors {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
