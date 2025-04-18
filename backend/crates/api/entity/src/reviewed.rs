use macros::entity;

use crate::user::UserId;

entity! {
    UserId -> Reviewed -> UserId {
        fields {
            score: u16,
            review: String,
        },
        upsert {
            score: u16,
            review: String,
        },
    }
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
