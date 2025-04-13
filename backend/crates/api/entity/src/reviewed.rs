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

impl UpsertReviewed {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Reviewed {
        Reviewed {
            id: self.get_id(),
            r#in: self.r#in,
            out: self.out,
            score: self.score,
            review: self.review,
        }
    }
}
