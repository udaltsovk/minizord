use entity::{
    member_of::{CreateMemberOf, MemberOf, MemberOfUpdate},
    team::TeamId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = TeamId,
    entity = MemberOf,
    create = CreateMemberOf,
    update = MemberOfUpdate,
    error = RepositoryError
)]
pub trait MemberOfRepository {}
