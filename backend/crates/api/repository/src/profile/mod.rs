use entity::profile;
use macros::urd_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    Profile
}
