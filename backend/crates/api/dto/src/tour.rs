use chrono::{DateTime, Utc};
use macros::dto;
use repository::{specialization::SpecializationId, tour::Tour as TourEntity};
use ulid::Ulid;
use utils::validation::RE_SENTENCE;

dto! {
    ///
    Tour {
        ///
        #[schema(format = Ulid)]
        id: Ulid,
        fields {
            ///
            #[validate(length(min = 1, max = 30), regex(path = *RE_SENTENCE))]
            #[schema(min_length = 1, max_length = 30)]
            name: String,

            ///
            #[schema(format = DateTime)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            ends_at: DateTime<Utc>,

            ///
            #[validate(range(min = 1, max = 10))]
            #[schema(minimum = 1, maximum = 10)]
            max_members: u16,

            ///
            required_specializations: Vec<Ulid>,
        },
        create
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            name: String,

            ///
            #[schema(format = DateTime)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            ends_at: DateTime<Utc>,

            ///
            #[validate(range(min = 1, max = 10))]
            #[schema(minimum = 1, maximum = 10)]
            max_members: u16,

            ///
            required_specializations: Vec<Ulid>,
        },
        update
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            name: String,

            ///
            #[schema(format = DateTime)]
            starts_at: DateTime<Utc>,

            ///
            #[schema(format = DateTime)]
            ends_at: DateTime<Utc>,

            ///
            #[validate(range(min = 1, max = 10))]
            #[schema(minimum = 1, maximum = 10)]
            max_members: u16,

            ///
            required_specializations: Vec<Ulid>,
        }
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
