use actix_multipart::form::tempfile::TempFile;
use dto::image::Image;
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

const MAX_IMAGE_SIZE: usize = 5_976_883;

#[service(error = ServiceError)]
pub trait ProfileImageService {
    async fn upsert_by_id(
        &self,
        id: Ulid,
        file: TempFile,
        check_user: bool,
    ) -> ();

    async fn find_by_id(&self, id: Ulid, check_user: bool) -> Option<Image>;

    async fn get_by_id(&self, id: Ulid, check_user: bool) -> Image;

    async fn delete_by_id(&self, id: Ulid, check_user: bool) -> ();
}
