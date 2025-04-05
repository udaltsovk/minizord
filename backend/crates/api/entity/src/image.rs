use std::fmt;

use bytes::Bytes;
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageId(Ulid);
impl ImageId {
    #[cfg(feature = "s3")]
    pub const BUCKET: &'static str = "images";
}
impl fmt::Display for ImageId {
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
