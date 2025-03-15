use macros::repository;
use ulid::Ulid;

#[cfg(feature = "surrealdb")]
pub mod surreal;

repository! {
    Organizator {
        id: Ulid,
        fields {
            username: String,
            password_hash: String,
        },
        create {
            username: String,
            password_hash: String
        },
        update {
            username: String,
            password_hash: String,
        }
    } {
        find_by_username(&self, username: &str) -> Option<Organizator>;
        exists_by_username(&self, username: &str) -> bool;
    }
}

impl CreateOrganizator {
    fn into_entity(self) -> Organizator {
        Organizator {
            id: OrganizatorId::from(Ulid::new()),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
        }
    }
}
