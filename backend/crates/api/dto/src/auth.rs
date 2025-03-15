use macros::request;
use utils::validation::validate_password;

request! {
    ///
    Login {
        #[validate(length(min = 3, max = 20))]
        #[schema(min_length = 3, max_length = 20)]
        ///
        login: String,

        #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        ///
        password: String
    }

    ///
    PasswordChange {
        #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        ///
        current_password: String,

        #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        ///
        new_password: String,

        #[validate(
            length(min = 8, max = 100),
            must_match(other = "new_password"),
            custom(function = "validate_password")
        )]
        #[schema(format = Password, min_length = 8, max_length = 100)]
        ///
        new_password_repeat: String,
    }
}
