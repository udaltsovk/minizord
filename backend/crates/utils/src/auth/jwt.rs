use chrono::Utc;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

static TOKEN_LIFETIME: usize = 60 * 60 * 24 * 3_usize; // 3 days

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod test {
    use argon2::password_hash::SaltString;
    use chrono::Utc;
    use jsonwebtoken::Algorithm;
    use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
    use rstest::{fixture, rstest};
    use ulid::Ulid;

    use super::{new as new_token, parse as parse_claims};

    #[fixture]
    fn secret() -> String {
        SaltString::generate(&mut ChaCha20Rng::from_entropy()).to_string()
    }

    #[rstest]
    #[case::organizator("organizator")]
    #[case::mentor("organizator")]
    #[case::participant("organizator")]
    fn basic(#[case] entity: &str, secret: String) {
        let id = Ulid::new();
        let current_time =
            usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);

        let token = new_token(entity, id, &secret);

        let expected_header = jsonwebtoken::Header {
            typ: Some("JWT".to_string()),
            alg: Algorithm::HS256,
            kid: Some(entity.to_string()),
            ..Default::default()
        };
        let header = jsonwebtoken::decode_header(&token);
        assert_eq!(header, Ok(expected_header));

        let claims = parse_claims(&token, &secret);
        let expected_claims = super::Claims {
            exp: current_time.saturating_add(super::TOKEN_LIFETIME),
            iat: current_time,
            sub: id.to_string(),
        };
        assert_eq!(claims, Some(expected_claims));
    }

    #[rstest]
    #[case::claims((
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string(),
        secret()
    ))]
    #[case::secret((
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6Im9yZ2FuaXphdG9yIn0.eyJleHAiOjE2NDUwNzIyNDIsImlhdCI6MTY0NDgxMzA0Miwic3ViIjoiMDAwMDAwMDAwMDJZSjVQVFNXN1I2RENTVFEifQ.jhXavh7YqscFGFBkpFneXP9mYNaAXUYXNc432q2RQ2I".to_string(),
        secret()
    ))]
    #[case::expired((
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6Im9yZ2FuaXphdG9yIn0.eyJleHAiOjE2NDUwNzIyNDIsImlhdCI6MTY0NDgxMzA0Miwic3ViIjoiMDAwMDAwMDAwMDJZSjVQVFNXN1I2RENTVFEifQ.0vq_J1-Mp2SCwyrTLhJVzI5VBIya33XFK8_ghhRPC7Q".to_string(),
        secret()
    ))]
    #[should_panic]
    #[case::valid({
        let secret = secret();
        (new_token("organizator", Ulid::new(), &secret), secret)
    })]
    fn invalid(#[case] (token, secret): (String, String)) {
        let claims = super::parse(&token, &secret);
        assert!(claims.is_none());
    }
}
