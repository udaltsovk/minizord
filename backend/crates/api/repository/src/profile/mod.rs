use std::collections::HashMap;

use entity::profile;
use macros::urd_repository;

use crate::common::RepositoryError;

#[cfg(feature = "surrealdb")]
pub mod surreal;

urd_repository! {
    Profile
        Err: RepositoryError
    {
        async fn count_filled(&self) -> u32;
        async fn count_by_city(&self) -> HashMap<String, u32>;
    }
}
