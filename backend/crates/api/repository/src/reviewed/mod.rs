use macros::crud_repository;

use crate::user::UserId;

#[cfg(feature = "surrealdb")]
pub mod surreal;

crud_repository! {
    UserId -> Reviewed -> UserId {
        fields {
            score: u16,
            review: String,
        },
        create {
            score: u16,
            review: String,
        },
        update {
            score: u16,
            review: String,
        }
    }
}

impl CreateReviewed {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Reviewed {
        Reviewed {
            id: self.get_id_string(),
            r#in: self.r#in,
            out: self.out,
            score: self.score,
            review: self.review,
        }
    }
}
