use chrono::Utc;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

static TOKEN_LIFETIME: usize = 60 * 60 * 24 * 3_usize; // 3 days

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

#[tracing::instrument(skip_all, level = "debug")]
pub fn new(entity: &str, id: Ulid, secret: &str) -> String {
    let current_time =
        usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(entity.to_string());
    match encode(
        &header,
        &Claims {
            exp: current_time.saturating_add(TOKEN_LIFETIME),
            iat: current_time,
            sub: id.to_string(),
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(err) => panic!("{err}"),
    }
}

#[tracing::instrument(skip_all, level = "debug")]
pub fn parse(token: &str, secret: &str) -> Option<Claims> {
    match decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Err(..) => None,
        Ok(data) => Some(data.claims),
    }
}
