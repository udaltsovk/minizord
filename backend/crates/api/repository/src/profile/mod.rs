use macros::{RepositoryId, repository};
use ulid::Ulid;

use crate::user::UserId;

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    Profile {
        id: Ulid,
        fields {
            has_avatar: bool,
            name: String,
            surname: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        create {
            user: UserId,
            has_avatar: bool,
            name: String,
            surname: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        update {
            name: String,
            surname: String,
            city: String,
            bio: String,
            portfolio_urls: Vec<String>,
        }
    }
}

impl CreateProfile {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Profile {
        Profile {
            id: ProfileId::from(<UserId as Into<Ulid>>::into(self.user)),
            has_avatar: self.has_avatar,
            name: self.name,
            surname: self.surname,
            city: self.city,
            bio: self.bio,
            portfolio_urls: self.portfolio_urls,
        }
    }
}
