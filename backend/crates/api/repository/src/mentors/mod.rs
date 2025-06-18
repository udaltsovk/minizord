use entity::{
    mentors::{CreateMentors, Mentors, MentorsUpdate},
    team::TeamId,
    user::UserId,
};
use macros::repository;

use crate::common::RepositoryError;

pub mod surreal;

#[repository(
    r#in = UserId,
    out = TeamId,
    entity = Mentors,
    create = CreateMentors,
    update = MentorsUpdate,
    error = RepositoryError
)]
pub trait MentorsRepository {}
