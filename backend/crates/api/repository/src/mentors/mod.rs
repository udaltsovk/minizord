use macros::{crud_repository, entity};

use crate::{team::TeamId, user::UserId};

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    UserId -> Mentors -> TeamId { }
}

crud_repository! {
    UserId -> Mentors -> TeamId
}

impl CreateMentors {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Mentors {
        Mentors {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
        }
    }
}
