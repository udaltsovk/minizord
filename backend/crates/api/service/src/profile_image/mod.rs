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
        upsert_by_id(&self, id: Ulid, file: TempFile) -> ();
        find_by_id(&self, id: Ulid) -> Option<Image>;
        get_by_id(&self, id: Ulid) -> Image;
        delete_by_id(&self, id: Ulid) -> ();
    }
}
