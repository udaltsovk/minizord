use entity::{
    has_experience_as::{HasExperienceAs, UpsertHasExperienceAs},
    specialization::SpecializationId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = SpecializationId,
    entity = HasExperienceAs,
    upsert = UpsertHasExperienceAs,
    error = RepositoryError
)]
pub trait HasExperienceAsRepository {}
