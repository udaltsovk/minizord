use passwords::{analyzer::analyze, scorer::score};

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_password(password: &str, _: &()) -> garde::Result {
    score(&analyze(password))
        .ge(&70.0)
        .then_some(())
        .ok_or(garde::Error::new("password is too weak"))
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use passwords::PasswordGenerator;
    use rstest::{fixture, rstest};

    use super::validate_password;

    #[fixture]
    fn secure_password_generator() -> PasswordGenerator {
        PasswordGenerator {
            length: 20,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: true,
            strict: true,
        }
    }

    #[fixture]
    fn insecure_password_generator() -> PasswordGenerator {
        PasswordGenerator {
            length: 6,
            numbers: false,
            lowercase_letters: true,
            uppercase_letters: false,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        }
    }

    #[rstest]
    #[case::secure(secure_password_generator())]
    #[should_panic]
    #[case::insecure(insecure_password_generator())]
    fn basic(#[case] password_generator: PasswordGenerator) {
        for _ in 0..100 {
            let password = password_generator.generate_one().unwrap();
            let res = validate_password(&password, &());
            assert!(res.is_ok());
        }
    }
}
