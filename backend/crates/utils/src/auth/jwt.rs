use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

static TOKEN_LIFETIME: usize = 60 * 60 * 24 * 3 as usize; // 3 days

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

pub fn new(entity: &str, id: Ulid, secret: &str) -> String {
    let current_time = Utc::now().timestamp() as usize;
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(entity.to_string());
    encode(
        &header,
        &Claims {
            exp: current_time + TOKEN_LIFETIME,
            iat: current_time,
            sub: id.to_string(),
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn parse(token: &str, secret: &str) -> Option<Claims> {
    match decode(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Err(..) => None,
        Ok(data) => Some(data.claims),
    }
}
