use chrono::{DateTime, Utc};
use entity::{specialization::SpecializationId, tour::Tour as TourEntity};
use garde::Validate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utils::validation::RE_SENTENCE;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Debug)]
///
pub struct Tour {
    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub id: Ulid,

    ///
    #[schema(min_length = 1, max_length = 30)]
    pub name: String,

    ///
    #[schema(format = DateTime)]
    pub starts_at: DateTime<Utc>,

    ///
    #[schema(format = DateTime)]
    pub ends_at: DateTime<Utc>,

    ///
    #[schema(minimum = 1, maximum = 10, examples(5))]
    pub max_members: u16,

    ///
    pub required_specializations: Vec<Ulid>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct CreateTour {
    ///
    #[schema(min_length = 3, max_length = 20)]
    #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
    pub name: String,

    ///
    #[schema(format = DateTime)]
    #[garde(skip)]
    pub starts_at: DateTime<Utc>,

    ///
    #[schema(format = DateTime)]
    #[garde(skip)]
    pub ends_at: DateTime<Utc>,

    ///
    #[schema(minimum = 1, maximum = 10, examples(5))]
    #[garde(range(min = 1, max = 10))]
    pub max_members: u16,

    ///
    #[garde(skip)]
    pub required_specializations: Vec<Ulid>,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct TourUpdate {
    ///
    #[schema(min_length = 3, max_length = 20)]
    #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
    pub name: Option<String>,

    ///
    #[schema(format = DateTime)]
    #[garde(skip)]
    pub starts_at: Option<DateTime<Utc>>,

    ///
    #[schema(format = DateTime)]
    #[garde(skip)]
    pub ends_at: Option<DateTime<Utc>>,

    ///
    #[schema(minimum = 1, maximum = 10, examples(5))]
    #[garde(range(min = 1, max = 10))]
    pub max_members: Option<u16>,

    ///
    #[garde(skip)]
    pub required_specializations: Option<Vec<Ulid>>,
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
