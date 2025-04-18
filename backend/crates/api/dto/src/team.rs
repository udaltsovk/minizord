use entity::team::Team as TeamEntity;
use macros::dto;
use ulid::Ulid;
use utils::validation::RE_SENTENCE;

dto! {
    ///
    Team {
        fields {
            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            id: Ulid,

            ///
            #[schema(min_length = 1, max_length = 30)]
            name: String,

            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            lead: Ulid,

            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            tour: Ulid,
        },
        create
        ///
        {
            ///
            #[schema(min_length = 3, max_length = 20)]
            #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
            name: String,

            ///
            #[schema(format = Ulid, examples(Ulid::default))]
            #[garde(skip)]
            tour: Ulid,
        },
        update
        ///
        {
            ///
            #[schema(min_length = 3, max_length = 20)]
            #[garde(length(min = 3, max = 20), pattern(*RE_SENTENCE))]
            name: String
        },
    }
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
