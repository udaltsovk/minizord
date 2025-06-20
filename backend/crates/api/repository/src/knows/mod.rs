use entity::{
    knows::{Knows, UpsertKnows},
    technology::TechnologyId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = TechnologyId,
    entity = Knows,
    upsert = UpsertKnows,
    error = RepositoryError
)]
pub trait KnowsRepository {}
