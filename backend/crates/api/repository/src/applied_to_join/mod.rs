use entity::{
    applied_to_join::{
        AppliedToJoin, AppliedToJoinUpdate, CreateAppliedToJoin,
    },
    team::TeamId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = TeamId,
    entity = AppliedToJoin,
    create = CreateAppliedToJoin,
    update = AppliedToJoinUpdate,
    error = RepositoryError
)]
pub trait AppliedToJoinRepository {}
