use macros::dto;
use repository::profile::Profile as ProfileEntity;
use ulid::Ulid;
use utils::validation::validate_portfolio_urls;

dto! {
    ///
    Profile {
        ///
        #[schema(format = Ulid)]
        id: Ulid,
        fields {
            ///
            #[schema(min_length = 1, max_length = 128)]
            name: String,

            ///
            #[schema(min_length = 1, max_length = 128)]
            surname: String,

            //
            #[schema(min_length = 1, max_length = 128)]
            city: String,

            ///
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,

            ///
            #[schema(min_length = 0)]
            portfolio_urls: Vec<String>,
        },
        upsert
        ///
        {
            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            name: String,

            ///
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            surname: String,

            //
            #[validate(length(min = 1, max = 128))]
            #[schema(min_length = 1, max_length = 128)]
            city: String,

            ///
            #[validate(length(min = 0, max = 4096))]
            #[schema(min_length = 0, max_length = 4096)]
            bio: String,

            ///
            #[validate(length(min = 0), custom(function = "validate_portfolio_urls"))]
            #[schema(min_length = 0)]
            portfolio_urls: Vec<String>,
        },
    }
}

impl From<ProfileEntity> for Profile {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(entity: ProfileEntity) -> Self {
        Self {
            id: entity.id.into(),
            name: entity.name,
            surname: entity.surname,
            city: entity.city,
            bio: entity.bio,
            portfolio_urls: entity.portfolio_urls.clone(),
        }
    }
}
