use entity::{
    participated_in::{
        CreateParticipatedIn, ParticipatedIn, ParticipatedInUpdate,
    },
    tour::TourId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = TourId,
    entity = ParticipatedIn,
    create = CreateParticipatedIn,
    update = ParticipatedInUpdate,
    error = RepositoryError
)]
pub trait ParticipatedInRepository {}
