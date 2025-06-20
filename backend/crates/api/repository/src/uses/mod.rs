use entity::{
    team::TeamId,
    technology::TechnologyId,
    uses::{CreateUses, Uses, UsesUpdate},
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = TeamId,
    out = TechnologyId,
    entity = Uses,
    create = CreateUses,
    update = UsesUpdate,
    error = RepositoryError
)]
pub trait UsesRepository {}
