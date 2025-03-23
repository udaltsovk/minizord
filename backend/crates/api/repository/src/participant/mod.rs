use macros::repository;
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    Participant {
        id: Ulid,
        fields {
            email: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        create {
            email: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        },
        update {
            // email: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
            portfolio_urls: Vec<String>,
        }
    } {
        find_by_email(&self, email: &str) -> Option<Participant>;
        exists_by_email(&self, email: &str) -> bool;
    }
}

impl CreateParticipant {
    #[tracing::instrument(skip_all, level = "trace")]
    fn into_entity(self) -> Participant {
        Participant {
            id: ParticipantId::from(Ulid::new()),
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            name: self.name.clone(),
            surname: self.surname.clone(),
            bio: self.bio.clone(),
            portfolio_urls: self.portfolio_urls.clone(),
        }
    }
}
