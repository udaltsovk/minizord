use chrono::{DateTime, Utc};
use entity::{specialization::SpecializationId, tour::Tour as TourEntity};
use macros::dto;
use ulid::Ulid;
use utils::validation::RE_SENTENCE;

dto! {
    ///
    Tour {
        fields {
            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            #[garde(skip)]
            id: Ulid,

            ///
            #[schema(min_length = 1, max_length = 30)]
            #[garde(skip)]
            name: String,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            ends_at: DateTime<Utc>,

            ///
            #[schema(minimum = 1, maximum = 10, examples(5))]
            #[garde(skip)]
            max_members: u16,

            ///
            #[garde(skip)]
            required_specializations: Vec<Ulid>,
        },
        create
        ///
        {
            ///
            #[schema(min_length = 3, max_length = 20)]
            #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
            name: String,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            ends_at: DateTime<Utc>,

            ///
            #[schema(minimum = 1, maximum = 10, examples(5))]
            #[garde(range(min = 1, max = 10))]
            max_members: u16,

            ///
            #[garde(skip)]
            required_specializations: Vec<Ulid>,
        },
        update
        ///
        {
            ///
            #[schema(min_length = 3, max_length = 20)]
            #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
            name: String,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            #[garde(skip)]
            ends_at: DateTime<Utc>,

            ///
            #[schema(minimum = 1, maximum = 10, examples(5))]
            #[garde(range(min = 1, max = 10))]
            max_members: u16,

            ///
            #[garde(skip)]
            required_specializations: Vec<Ulid>,
        },
    }
}

impl From<TourEntity> for Tour {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: TourEntity) -> Self {
        Self {
            id: entity.id.into(),
            name: entity.name,
            starts_at: entity.starts_at,
            ends_at: entity.ends_at,
            max_members: entity.max_members,
            required_specializations: entity
                .required_specializations
                .into_iter()
                .map(SpecializationId::into)
                .collect(),
        }
    }
}
