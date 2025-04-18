use entity::reviewed::Reviewed;
use macros::dto;
use ulid::Ulid;

dto! {
    ///
    Review {
        fields {
            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            reviewer_id: Ulid,

            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            reviewee_id: Ulid,

            ///
            #[schema(minimum = 0, maximum = 10, examples(7))]
            score: u16,

            ///
            #[schema(max_length = 4096)]
            review: String,
        },
        upsert
        ///
        {
            ///
            #[schema(minimum = 0, maximum = 10, examples(7))]
            #[garde(range(min = 0, max = 10))]
            score: u16,

            ///
            #[schema(max_length = 4096)]
            #[garde(length(max = 4096))]
            review: String,
        },
    }
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
