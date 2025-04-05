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

impl CreateTechnology {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self) -> Technology {
        Technology {
            id: TechnologyId::from(Ulid::new()),
            name: self.name.clone(),
        }
    }
}
