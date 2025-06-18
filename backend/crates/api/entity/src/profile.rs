use macros::entity;
use ulid::Ulid;

use crate::EntityId;

#[entity]
pub struct Profile {
    pub id: Ulid,

    #[field]
    #[upsert]
    pub name: String,

    #[field]
    #[upsert]
    pub surname: String,

    #[field]
    #[upsert]
    pub telegram: String,

    #[field]
    #[upsert]
    pub city: String,

    #[field]
    #[upsert]
    pub bio: String,

    #[field]
    #[upsert]
    pub portfolio_urls: Vec<String>,

    #[field]
    #[upsert]
    pub has_avatar: bool,
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
