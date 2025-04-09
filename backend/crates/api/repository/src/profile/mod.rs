use entity::profile;
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    Profile
        Err: RepositoryError
}
