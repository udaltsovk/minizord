use std::sync::LazyLock;

use application::service::token::TokenService;
use domain::session::{
    Session, SessionTokenPair, access_token::SessionAccessToken,
    refresh_token::SessionRefreshToken,
};
pub use jsonwebtoken::errors::Error as JwtAdapterError;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use lib::{domain::Id, instrument_all};
use tap::{Conv as _, Pipe as _, Tap as _};

use crate::claims::{Claims, TokenKind};

mod claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

const ALGORITHM: Algorithm = Algorithm::HS512;

static TOKEN_VALIDATION: LazyLock<Validation> = LazyLock::new(|| {
    Validation::new(ALGORITHM).tap_mut(|validation| {
        validation.required_spec_claims = claims::CLAIMS_FIELDS.clone();
    })
});

impl JwtService {
    fn generate(
        &self,
        session_id: Id<Session>,
        kind: TokenKind,
    ) -> Result<String, JwtAdapterError> {
        encode(
            &Header::new(ALGORITHM),
            &Claims::from((session_id, kind)),
            &self.encoding_key,
        )
    }
}

#[instrument_all("JwtService")]
impl TokenService for JwtService {
    type AdapterError = JwtAdapterError;

    fn generate_pair(
        &self,
        session_id: Id<Session>,
    ) -> Result<SessionTokenPair, Self::AdapterError> {
        let access_token = self
            .generate(session_id, TokenKind::Access)
            .map(SessionAccessToken::from)?;

        let refresh_token = self
            .generate(session_id, TokenKind::Refresh)
            .map(SessionRefreshToken::from)?;

        SessionTokenPair {
            access_token,
            refresh_token,
        }
        .pipe(Ok)
    }

    fn parse(&self, token: &str) -> Result<Id<Session>, Self::AdapterError> {
        decode::<Claims>(token, &self.decoding_key, &TOKEN_VALIDATION)?
            .claims
            .conv::<Id<_>>()
            .pipe(Ok)
    }
}

#[instrument_all("JwtService")]
impl JwtService {
    pub fn new(secret: &str) -> Self {
        let secret = secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}
