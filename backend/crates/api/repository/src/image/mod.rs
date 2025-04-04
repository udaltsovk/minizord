use std::fmt::{self, Display};

use bytes::Bytes;
use macros::urd_repository;
use ulid::Ulid;

#[cfg(feature = "s3")]
pub mod s3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageId(Ulid);
impl ImageId {
    #[cfg(feature = "s3")]
    const BUCKET: &'static str = "images";
}
impl Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl From<Ulid> for ImageId {
    fn from(id: Ulid) -> Self {
        Self(id)
    }
}
impl From<ImageId> for Ulid {
    fn from(id: ImageId) -> Self {
        id.0
    }
}

#[derive(Debug)]
pub struct Image {
    pub id: ImageId,
    pub content_type: String,
    pub data: Bytes,
}

#[derive(Debug)]
pub struct UpsertImage {
    pub content_type: String,
    pub data: Bytes,
}

urd_repository! {
    Image
}
