use macros::entity;
use ulid::Ulid;

entity! {
    Profile {
        id: Ulid,
        fields {
            name: String,
            surname: String,
            telegram: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
            has_avatar: bool,
        },
        upsert {
            name: String,
            surname: String,
            telegram: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
            has_avatar: bool,
        },
    }
}

impl From<(UpsertProfile, ProfileId)> for Profile {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from((upsert_entity, id): (UpsertProfile, ProfileId)) -> Self {
        Self {
            id,
            name: upsert_entity.name,
            surname: upsert_entity.surname,
            telegram: upsert_entity.telegram,
            city: upsert_entity.city,
            bio: upsert_entity.bio,
            portfolio_urls: upsert_entity.portfolio_urls,
            has_avatar: upsert_entity.has_avatar,
        }
    }
}
