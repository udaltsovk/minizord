use macros::dto;
use repository::organizator::Organizator as OrganizatorEntity;
use ulid::Ulid;
use utils::validation::{RE_USERNAME, validate_password};

dto! {
    ///
    Organizator {
        ///
        id: Ulid,
        fields {
            ///
            #[validate(length(min = 3, max = 20), regex(path = *RE_USERNAME))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,
        },
        create
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,

            ///
            #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
            #[schema(format = Password, min_length = 8, max_length = 100)]
            password: String,
        },
        update
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            username: String
        }
    }
}

impl From<OrganizatorEntity> for Organizator {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: OrganizatorEntity) -> Self {
        Self {
            id: entity.id.into(),
            username: entity.username.into(),
        }
    }
}
