use macros::repository;
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    Mentor {
        id: Ulid,
        fields {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
        },
        create {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
        },
        update {
            username: String,
            password_hash: String,
            name: String,
            surname: String,
            bio: String,
        }
    } {
        find_by_username(&self, username: &str) -> Option<Mentor>;
        exists_by_username(&self, username: &str) -> bool;
    }
}

impl CreateMentor {
    #[tracing::instrument(skip_all)]
    fn into_entity(self) -> Mentor {
        Mentor {
            id: MentorId::from(Ulid::new()),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            name: self.name.clone(),
            surname: self.surname.clone(),
            bio: self.bio.clone(),
        }
    }
}
