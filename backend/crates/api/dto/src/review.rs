use entity::reviewed::Reviewed;
use garde::Validate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Debug)]
///
pub struct Review {
    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub reviewer_id: Ulid,

    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub reviewee_id: Ulid,

    ///
    #[schema(minimum = 0, maximum = 10, examples(7))]
    pub score: u16,

    ///
    #[schema(max_length = 4096)]
    pub review: String,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct UpsertReview {
    ///
    #[schema(minimum = 0, maximum = 10, examples(7))]
    #[garde(range(min = 0, max = 10))]
    pub score: u16,

    ///
    #[schema(max_length = 4096)]
    #[garde(length(max = 4096))]
    pub review: String,
}

impl From<Reviewed> for Review {
    fn from(entity: Reviewed) -> Self {
        Self {
            reviewer_id: entity.r#in.into(),
            reviewee_id: entity.out.into(),
            score: entity.score,
            review: entity.review,
        }
    }
}
