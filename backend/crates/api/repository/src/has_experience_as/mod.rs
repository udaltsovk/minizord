use entity::{
    has_experience_as, specialization::SpecializationId, user::UserId,
};
use macros::urd_repository;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    UserId -> has_experience_as -> SpecializationId
}
