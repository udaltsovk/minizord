use std::{any::type_name, marker::PhantomData};

use serde::de::DeserializeOwned;
use surrealdb::Value;

use crate::Surreal;

mod user;

#[derive(Clone)]
pub struct SurrealRepositoryImpl<T: Send + Sync> {
    pool: Surreal,
    _entity: PhantomData<T>,
}

impl<T: Send + Sync> SurrealRepositoryImpl<T> {
    pub fn new(db: &Surreal) -> Self {
        Self {
            pool: db.clone(),
            _entity: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! surql_query {
    ($path:literal) => {
        include_str!(concat!("../../db/queries/", $path, ".surql"))
    };
}

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
