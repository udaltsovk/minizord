use macros::urd_repository;
use ulid::Ulid;

#[cfg(feature = "s3")]
pub mod s3;

pub struct ImageId(pub Ulid);

pub struct Image {
    pub id: ImageId,
    pub data: Vec<u8>,
}

pub struct UpsertImage {
    pub data: Vec<u8>,
}

urd_repository! {
    Image
}
