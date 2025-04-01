use dto::profile::{CreateProfile, Profile, ProfileUpdate};
use macros::service;
use ulid::Ulid;

pub mod implementation;

service! {
    Profile {
        create(&self, id: Ulid, new: CreateProfile) -> Profile;
        find_by_id(&self, id: Ulid) -> Option<Profile>;
        get_by_id(&self, id: Ulid) -> Profile;
        update_by_id(&self, id: Ulid, update: ProfileUpdate) -> Profile;
        delete_by_id(&self, id: Ulid) -> ();
    }
}
