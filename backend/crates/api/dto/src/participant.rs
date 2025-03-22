use macros::dto;
use repository::participant::Participant as ParticipantEntity;
use ulid::Ulid;
use utils::validation::{RE_USERNAME, validate_password, validate_portfolio_urls};

dto! {
    ///
    Participant {
        ///
        id: Ulid,
        fields {
            #[validate(length(min = 3, max = 20), regex(path = *RE_USERNAME))]
            #[schema(min_length = 3, max_length = 20)]
            ///
            username: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            name: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            surname: String,

            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            ///
            bio: String,

            #[validate(length(min = 0), custom(function = "validate_portfolio_urls"))]
            #[schema(min_length = 0)]
            ///
            portfolio_urls: Vec<String>,
        },
        create
        ///
        {
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            ///
            username: String,

            #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
            #[schema(format = Password, min_length = 8, max_length = 100)]
            ///
            password: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            name: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            surname: String,

            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            ///
            bio: String,

            #[validate(length(min = 0), custom(function = "validate_portfolio_urls"))]
            #[schema(min_length = 0)]
            ///
            portfolio_urls: Vec<String>,
        },
        update
        ///
        {
            #[validate(length(min = 3, max = 20))]
            #[schema(min_length = 3, max_length = 20)]
            ///
            username: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            name: String,

            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            ///
            surname: String,

            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            ///
            bio: String,

            #[validate(length(min = 0), custom(function = "validate_portfolio_urls"))]
            #[schema(min_length = 0)]
            ///
            portfolio_urls: Vec<String>,
        }
    }
}

impl From<ParticipantEntity> for Participant {
    #[tracing::instrument]
    fn from(entity: ParticipantEntity) -> Self {
        Self {
            id: entity.id.into(),
            username: entity.username.into(),
            name: entity.name.into(),
            surname: entity.surname.into(),
            bio: entity.bio.into(),
            portfolio_urls: entity.portfolio_urls.clone(),
        }
    }
}
