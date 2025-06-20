use macros::relation;

use crate::{EntityId, user::UserId};

#[relation]
pub struct Reviewed {
    pub r#in: UserId,
    pub out: UserId,

    #[field]
    #[upsert]
    pub score: u16,

    #[field]
    #[upsert]
    pub review: String,
}

impl From<UpsertReviewed> for Reviewed {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(upsert_relation: UpsertReviewed) -> Self {
        Self {
            id: upsert_relation.get_id(),
            r#in: upsert_relation.r#in,
            out: upsert_relation.out,
            score: upsert_relation.score,
            review: upsert_relation.review,
        }
    }
}
