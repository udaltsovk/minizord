use dto::profile::{Profile, UpsertProfile};
use macros::service;
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    Profile
        Err: ServiceError
    {
        upsert_by_id(&self, id: Ulid, object: UpsertProfile, has_avatar: Option<bool>) -> Profile;
        find_by_id(&self, id: Ulid) -> Option<Profile>;
        get_by_id(&self, id: Ulid) -> Profile;
        delete_by_id(&self, id: Ulid) -> ();
    }
}
