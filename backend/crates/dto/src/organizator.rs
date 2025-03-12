use macros::dto;
use repository::organizator::Organizator as OrganizatorEntity;
use ulid::Ulid;

dto! {
    Organizator {
        id: Ulid,
        fields {
            username: String,
        },
        create {
            username: String,
            password: String,
        },
        update {
            username: String
        }
    }
}

impl From<OrganizatorEntity> for Organizator {
    fn from(entity: OrganizatorEntity) -> Self {
        Self {
            id: entity.id.into(),
            username: entity.username.into(),
        }
    }
}
