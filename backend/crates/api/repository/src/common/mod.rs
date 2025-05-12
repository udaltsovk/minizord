use std::any::type_name;

pub use error::RepositoryError;
use serde::de::DeserializeOwned;
use surrealdb::Value;

mod error;

// TODO: remove when surrealdb 3.0.0 arrives
pub(crate) trait ExtractValue {
    fn extract<T: DeserializeOwned>(self) -> T;
}
impl ExtractValue for Value {
    fn extract<T: DeserializeOwned>(self) -> T {
        serde_json::from_value(self.into_inner().into_json())
            .unwrap_or_else(|_| panic!("Expected a valid {}", type_name::<T>()))
    }
}
