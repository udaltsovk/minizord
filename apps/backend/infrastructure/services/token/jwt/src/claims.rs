use std::{collections::HashSet, sync::LazyLock};

use chrono::Utc;
use domain::session::Session;
use lib::domain::Id;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum TokenKind {
    Access,
    Refresh,
}

impl TokenKind {
    fn lifetime(&self) -> usize {
        use TokenKind as T;
        match self {
            T::Access => 20 * 60,            // 20 minutes
            T::Refresh => 20 * 24 * 60 * 60, // 20 days
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: Uuid,
}

pub(super) static CLAIMS_FIELDS: LazyLock<HashSet<String>> =
    LazyLock::new(|| {
        HashSet::from_iter(["exp", "iat", "sub"].iter().map(|f| f.to_string()))
    });

impl From<(Id<Session>, TokenKind)> for Claims {
    fn from((id, kind): (Id<Session>, TokenKind)) -> Self {
        let current_time =
            usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);
        Self {
            exp: current_time.saturating_add(kind.lifetime()),
            iat: current_time,
            sub: id.into(),
        }
    }
}

impl From<Claims> for Id<Session> {
    fn from(cl: Claims) -> Self {
        cl.sub.into()
    }
}
