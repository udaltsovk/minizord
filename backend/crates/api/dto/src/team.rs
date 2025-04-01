use macros::dto;
use repository::team::Team as TeamEntity;
use ulid::Ulid;
use utils::validation::RE_SENTENCE;

dto! {
    ///
    Team {
        ///
        #[schema(format = Ulid)]
        id: Ulid,
        fields {
            ///
            #[validate(length(min = 1, max = 30), regex(path = *RE_SENTENCE))]
            #[schema(min_length = 1, max_length = 30)]
            name: String,

            ///
            #[schema(format = Ulid)]
            lead: Ulid,

            ///
            #[schema(format = Ulid)]
            tour: Ulid,
        },
        create
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            name: String,

            ///
            #[schema(format = Ulid)]
            tour: Ulid,
        },
        update
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            name: String
        }
    }
}

impl From<TeamEntity> for Team {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: TeamEntity) -> Self {
        Self {
            id: entity.id.into(),
            name: entity.name,
            lead: entity.lead,
            tour: entity.tour,
        }
    }
}
