use actix_multipart::form::tempfile::TempFile;
use dto::{
    image::Image,
    profile::{Profile, UpsertProfile},
};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Profile {
        upsert_by_id(&self, id: Ulid, object: UpsertProfile) -> Profile;
        find_by_id(&self, id: Ulid) -> Option<Profile>;
        get_by_id(&self, id: Ulid) -> Profile;
        delete_by_id(&self, id: Ulid) -> ();
        upsert_image_by_id(&self, id: Ulid, file: TempFile) -> ();
        find_image_by_id(&self, id: Ulid) -> Option<Image>;
        get_image_by_id(&self, id: Ulid) -> Image;
        delete_image_by_id(&self, id: Ulid) -> ();
    }
}
