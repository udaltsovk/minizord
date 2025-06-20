use std::sync::Arc;

pub use argon2::password_hash::Error;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash,
    PasswordHasher as PasswordHasherTrait, PasswordVerifier, Version,
    password_hash::SaltString,
};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};

#[derive(Clone)]
pub struct PasswordHasher {
    hasher: Arc<Argon2<'static>>,
}
impl PasswordHasher {
    #[tracing::instrument(
        name = "PasswordHasher::new",
        skip_all,
        level = "debug"
    )]
    pub fn new() -> Self {
        Self {
            hasher: Arc::new(Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(19_456, 1, 4, Some(32)).unwrap_or_default(),
            )),
        }
    }

    #[tracing::instrument(
        name = "PasswordHasher::hash",
        skip_all,
        level = "debug"
    )]
    pub fn hash(&self, password: &str) -> Result<String, Error> {
        let password_hash = self
            .hasher
            .hash_password(password.as_bytes(), &Self::gen_salt())?
            .to_string();
        Ok(password_hash)
    }

    #[tracing::instrument(
        name = "PasswordHasher::verify",
        skip_all,
        level = "debug"
    )]
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
impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use passwords::PasswordGenerator;
    use rstest::{fixture, rstest};

    use super::PasswordHasher;

    #[fixture]
    fn password() -> String {
        PasswordGenerator::new().generate_one().unwrap()
    }

    #[fixture]
    fn hasher<'a>() -> PasswordHasher {
        PasswordHasher::new()
    }

    #[rstest]
    fn basic(password: String, hasher: PasswordHasher) {
        let hashed = hasher.hash(&password).unwrap();

        assert!(hasher.verify(&password, &hashed).is_ok())
    }

    #[rstest]
    #[case::invalid((password(), "sdfsfdsfsfssafasfasdfsdfsfsfsdfsaf".to_string()))]
    #[should_panic]
    #[case::valid({
        let password = password();
        (password.clone(), hasher().hash(&password).unwrap())
    })]
    fn invalid(#[case] (password, password_hash): (String, String)) {
        let res = hasher().verify(&password, &password_hash);
        assert!(res.is_err());
    }
}
