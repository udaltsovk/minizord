use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    sync::Arc,
};

use serde::Serialize;
pub use service::common::wrapper::*;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, Debug)]
#[schema(value_type = String)]
pub struct BaseApiUrl(Arc<str>);
impl Deref for BaseApiUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl From<String> for BaseApiUrl {
    fn from(base_api_url: String) -> Self {
        Self(Arc::from(base_api_url))
    }
}
impl<'a> From<&'a str> for BaseApiUrl {
    fn from(base_api_url: &'a str) -> Self {
        Self(Arc::from(base_api_url))
    }
}
impl Display for BaseApiUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
