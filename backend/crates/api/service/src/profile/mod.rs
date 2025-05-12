use dto::profile::{Profile, UpsertProfile};
use macros::{metric_name, service};
use ulid::Ulid;

use crate::common::ServiceError;

pub mod implementation;

service! {
    Profile
        Err: ServiceError
    {
        async fn upsert_by_id(
            &self,
            id: Ulid,
            object: UpsertProfile,
            has_avatar: Option<bool>,
            check_user: bool
        ) -> Profile;

        async fn find_by_id(&self, id: Ulid, check_user: bool) -> Option<Profile>;

        async fn get_by_id(&self, id: Ulid, check_user: bool) -> Profile;

        async fn delete_by_id(&self, id: Ulid, check_user: bool) -> ();

        async fn init_metrics(&self);
    }
}

metric_name!(PROFILES_FILLED, "profiles_filled");
metric_name!(PROFILES_BY_CITY, "profiles_by_city");
