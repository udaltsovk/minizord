use lib::domain::validation::constraints::Constraint;
use passwords::{analyzer::analyze, scorer::score};

pub struct SecurePassword;

impl Constraint<String> for SecurePassword {
    fn check(&self, value: &String) -> bool {
        score(&analyze(value)).ge(&70.0)
    }

    fn error_msg(&self) -> String {
        "is not strong enough".to_string()
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use passwords::PasswordGenerator;
    use rstest::{fixture, rstest};

    use super::{Constraint as _, SecurePassword};

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
    fn intsecure_password_generator() -> PasswordGenerator {
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

    #[fixture]
    fn password_constrain() -> SecurePassword {
        SecurePassword
    }

    #[rstest]
    #[case::secure(secure_password_generator())]
    #[should_panic]
    #[case::intsecure(intsecure_password_generator())]
    fn basic(
        #[case] password_generator: PasswordGenerator,
        password_constrain: SecurePassword,
    ) {
        for _ in 0..100 {
            let password = password_generator.generate_one().unwrap();
            let res = password_constrain.check(&password);
            assert!(res);
        }
    }
}
