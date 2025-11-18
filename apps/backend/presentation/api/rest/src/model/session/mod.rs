use domain::session::{Session, SessionTokenPair, VerboseSession};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::user::JsonUser;

///
#[derive(Serialize, ToSchema)]
pub struct JsonSession {
    ///
    pub id: Uuid,
}

impl From<Session> for JsonSession {
    fn from(s: Session) -> Self {
        Self {
            id: s.id.into(),
        }
    }
}

///
#[derive(Serialize, ToSchema)]
pub struct JsonSessionTokenPair {
    ///
    pub access_token: String,
    ///
    pub refresh_token: String,
}

impl From<SessionTokenPair> for JsonSessionTokenPair {
    fn from(p: SessionTokenPair) -> Self {
        Self {
            access_token: p.access_token.into(),
            refresh_token: p.refresh_token.into(),
        }
    }
}

///
#[derive(Serialize, ToSchema)]
pub struct JsonVerboseSession {
    ///
    pub id: Uuid,
    ///
    #[serde(flatten)]
    pub token_pair: JsonSessionTokenPair,
    ///
    pub user: JsonUser,
}

impl From<VerboseSession> for JsonVerboseSession {
    fn from(s: VerboseSession) -> Self {
        Self {
            id: s.id.into(),
            token_pair: s.token_pair.into(),
            user: s.user.into(),
        }
    }
}
