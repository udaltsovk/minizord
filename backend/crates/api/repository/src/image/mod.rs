use entity::image::{Image, ImageId, UpsertImage};
use macros::repository;

use crate::common::RepositoryError;

pub mod s3;

#[repository(
    entity = Image,
    entity_id = ImageId,
    upsert = UpsertImage,
    error = RepositoryError
)]
pub trait ImageRepository {}
