use macros::entity;
use ulid::Ulid;

entity! {
    Technology {
        id: Ulid,
        fields {
            name: String,
        },
        create {
            name: String,
        },
        update {
            name: String,
        }
    }
}

impl From<CreateTechnology> for Technology {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(create_entity: CreateTechnology) -> Self {
        Self {
            id: TechnologyId::from(Ulid::new()),
            name: create_entity.name,
        }
    }
}
