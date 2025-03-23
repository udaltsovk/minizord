pub use argon2::password_hash::Error;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash,
    PasswordHasher as PasswordHasherTrait, PasswordVerifier, Version,
    password_hash::SaltString,
};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};

#[derive(Clone)]
pub struct PasswordHasher<'a> {
    hasher: Argon2<'a>,
}
impl PasswordHasher<'_> {
    #[tracing::instrument(skip_all, level = "debug")]
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(19_456, 1, 4, Some(32)).unwrap_or_default(),
            ),
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub fn hash(&self, password: &str) -> Result<String, Error> {
        let password_hash = self
            .hasher
            .hash_password(password.as_bytes(), &Self::gen_salt())?
            .to_string();
        Ok(password_hash)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub fn verify(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<(), Error> {
        self.hasher.verify_password(
            password.as_bytes(),
            &PasswordHash::new(password_hash)?,
        )
    }

    #[tracing::instrument(level = "debug")]
    fn gen_salt() -> SaltString {
        SaltString::generate(&mut ChaCha20Rng::from_entropy())
    }
}
