use macros::{RepositoryId, entity, urd_repository};
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

entity! {
    Profile {
        id: Ulid,
        fields {
            name: String,
            surname: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
            has_avatar: bool,
        },
        upsert {
            name: String,
            surname: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
            has_avatar: bool,
        },
    }
}

urd_repository! {
    Profile
}

impl UpsertProfile {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self, id: ProfileId) -> Profile {
        Profile {
            id,
            name: self.name,
            surname: self.surname,
            city: self.city,
            bio: self.bio,
            portfolio_urls: self.portfolio_urls,
            has_avatar: self.has_avatar,
        }
    }
}
