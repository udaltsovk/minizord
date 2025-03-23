use macros::dto;
use repository::mentor::Mentor as MentorEntity;
use ulid::Ulid;
use utils::validation::{RE_USERNAME, validate_password};

dto! {
    ///
    Mentor {
        ///
        id: Ulid,
        fields {
            ///
            #[validate(length(min = 3, max = 20), regex(path = *RE_USERNAME))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            name: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            surname: String,

            ///
            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,
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

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            name: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            surname: String,

            ///
            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,
        },
        update
        ///
        {
            ///
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            username: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            name: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            surname: String,

            ///
            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,
        }
    }
}

impl From<MentorEntity> for Mentor {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: MentorEntity) -> Self {
        Self {
            id: entity.id.into(),
            username: entity.username,
            name: entity.name,
            surname: entity.surname,
            bio: entity.bio,
        }
    }
}
