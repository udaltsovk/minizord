use macros::entity;
use ulid::Ulid;

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

impl UpsertProfile {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn into_entity(self, id: ProfileId) -> Profile {
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
