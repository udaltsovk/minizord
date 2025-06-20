use entity::team::Team as TeamEntity;
use garde::Validate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utils::validation::RE_SENTENCE;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Debug)]
///
pub struct Team {
    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub id: Ulid,

    ///
    #[schema(min_length = 1, max_length = 30)]
    pub name: String,

    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub lead: Ulid,

    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    pub tour: Ulid,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct CreateTeam {
    ///
    #[schema(min_length = 3, max_length = 20)]
    #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
    pub name: String,

    ///
    #[schema(format = Ulid, examples(Ulid::default))]
    #[garde(skip)]
    pub tour: Ulid,
}

#[derive(Deserialize, ToSchema, Validate, Clone, PartialEq, Debug)]
///
pub struct TeamUpdate {
    ///
    #[schema(min_length = 3, max_length = 20)]
    #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
    pub name: Option<String>,
}

impl From<TeamEntity> for Team {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: TeamEntity) -> Self {
        Self {
            id: entity.id.into(),
            name: entity.name,
            lead: entity.lead.into(),
            tour: entity.tour.into(),
        }
    }
}
