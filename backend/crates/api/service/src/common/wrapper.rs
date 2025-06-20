use std::{ops::Deref, sync::Arc};

use macros::Secret;

#[derive(Secret, Clone)]
pub struct JwtSecret(Arc<str>);
impl Deref for JwtSecret {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl From<String> for JwtSecret {
    fn from(token: String) -> Self {
        Self(Arc::from(token))
    }
}
impl<'a> From<&'a str> for JwtSecret {
    fn from(token: &'a str) -> Self {
        Self(Arc::from(token))
    }
}
