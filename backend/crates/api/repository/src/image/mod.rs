use entity::image;
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "s3")]
pub mod s3;

urd_repository! {
    Image
        Err: RepositoryError
}
