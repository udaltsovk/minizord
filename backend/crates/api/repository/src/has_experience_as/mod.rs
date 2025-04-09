use entity::{
    has_experience_as, specialization::SpecializationId, user::UserId,
};
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    UserId -> has_experience_as -> SpecializationId
        Err: RepositoryError
}
