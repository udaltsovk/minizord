use entity::image;
use macros::urd_repository;

#[cfg(feature = "s3")]
pub mod s3;

urd_repository! {
    Image
}
