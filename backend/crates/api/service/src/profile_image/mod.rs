use actix_multipart::form::tempfile::TempFile;
use dto::image::Image;
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    ProfileImage
        Err: ServiceError
    {
        async fn upsert_by_id(&self, id: Ulid, file: TempFile) -> ();
        async fn find_by_id(&self, id: Ulid) -> Option<Image>;
        async fn get_by_id(&self, id: Ulid) -> Image;
        async fn delete_by_id(&self, id: Ulid) -> ();
    }
}
