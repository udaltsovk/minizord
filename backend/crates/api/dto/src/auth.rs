use macros::request;
use utils::validation::validate_password;

request! {
    ///
    Login {
        ///
        #[garde(length(min = 3, max = 20))]
        #[schema(min_length = 3, max_length = 20)]
        email: String,

        ///
        #[garde(length(min = 8, max = 100), custom(validate_password))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        password: String
    }

    ///
    PasswordChange {
        ///
        #[garde(length(min = 8, max = 100), custom(validate_password))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        current_password: String,

        ///
        #[garde(length(min = 8, max = 100), custom(validate_password))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        new_password: String,

        ///
        #[garde(
            length(min = 8, max = 100),
            matches(new_password),
            custom(validate_password)
        )]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        new_password_repeat: String,
    }
}
