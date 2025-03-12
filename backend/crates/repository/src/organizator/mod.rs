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

    }
}
