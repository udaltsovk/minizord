use dto::profile::{Profile, UpsertProfile};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Profile {
        upsert_by_id(&self, id: Ulid, object: UpsertProfile) -> Profile;
        find_by_id(&self, id: Ulid) -> Option<Profile>;
        get_by_id(&self, id: Ulid) -> Profile;
        delete_by_id(&self, id: Ulid) -> ();
    }
}
