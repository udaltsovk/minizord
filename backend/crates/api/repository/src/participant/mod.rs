use macros::repository;
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    Participant {
        id: Ulid,
        fields {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        create {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        update {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        }
    } {
        find_by_username(&self, username: &str) -> Option<Participant>;
        exists_by_username(&self, username: &str) -> bool;
    }
}

impl CreateParticipant {
    #[tracing::instrument(skip_all)]
    fn into_entity(self) -> Participant {
        Participant {
            id: ParticipantId::from(Ulid::new()),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            name: self.name.clone(),
            surname: self.surname.clone(),
            bio: self.bio.clone(),
            portfolio_urls: self.portfolio_urls.clone(),
        }
    }
}
