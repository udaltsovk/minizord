use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher as PasswordHasherTrait,
    PasswordVerifier, Version, password_hash::SaltString,
};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};

pub use argon2::password_hash::Error;

pub struct PasswordHasher<'a> {
    hasher: Argon2<'a>,
}
impl PasswordHasher<'_> {
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(19_456, 16, 4, Some(16)).unwrap_or_default(),
            ),
        }
    }
    pub fn hash(&self, password: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
        let password_hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify(&self, password: &str, password_hash: &str) -> Result<(), Error> {
        self.hasher
            .verify_password(password.as_bytes(), &PasswordHash::new(password_hash)?)
    }
}
