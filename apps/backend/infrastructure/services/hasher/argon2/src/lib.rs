use application::service::hasher::HasherService;
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, PasswordHash,
    PasswordHasher as _, PasswordVerifier as _, Version,
    password_hash::{SaltString, rand_core::OsRng},
};
use lib::instrument_all;

pub struct Argon2Service {
    hasher: Argon2<'static>,
}

#[instrument_all("Argon2Service")]
impl HasherService for Argon2Service {
    type AdapterError = argon2::password_hash::Error;

    fn hash(&self, password: &[u8]) -> Result<String, Self::AdapterError> {
        self.hasher
            .hash_password(password, &Self::gen_salt())
            .map(|hashed| hashed.to_string())
    }

    fn verify(
        &self,
        password: &[u8],
        original_hash: &str,
    ) -> Result<(), Self::AdapterError> {
        self.hasher
            .verify_password(password, &PasswordHash::new(original_hash)?)
    }
}

#[instrument_all("Argon2Service")]
impl Argon2Service {
    #[tracing::instrument(level = "trace")]
    fn params() -> Params {
        ParamsBuilder::new()
            .m_cost(19_456)
            .t_cost(1)
            .p_cost(4)
            .output_len(32)
            .build()
            .expect("hasher params to be valid")
    }

    #[tracing::instrument(level = "trace")]
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Self::params(),
            ),
        }
    }

    #[tracing::instrument(level = "trace")]
    pub fn new_with_secret(secret: &'static [u8]) -> argon2::Result<Self> {
        Ok(Self {
            hasher: Argon2::new_with_secret(
                secret,
                Algorithm::Argon2id,
                Version::V0x13,
                Self::params(),
            )?,
        })
    }

    #[tracing::instrument(level = "debug")]
    fn gen_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

impl Default for Argon2Service {
    fn default() -> Self {
        Self::new()
    }
}
