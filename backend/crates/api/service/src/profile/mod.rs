use dto::profile::{Profile, UpsertProfile};
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    Profile
        Err: ServiceError
    {
        async fn upsert_by_id(&self, id: Ulid, object: UpsertProfile, has_avatar: Option<bool>) -> Profile;
        async fn find_by_id(&self, id: Ulid) -> Option<Profile>;
        async fn get_by_id(&self, id: Ulid) -> Profile;
        async fn delete_by_id(&self, id: Ulid) -> ();
    }
}
