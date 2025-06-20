use garde::Validate;
use serde::Deserialize;
use utils::validation::validate_password;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Validate, Debug, Clone)]
///
pub struct LoginRequest {
    ///
    #[garde(length(min = 3, max = 20))]
    #[schema(min_length = 3, max_length = 20)]
    pub email: String,

    ///
    #[garde(length(min = 8, max = 100), custom(validate_password))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub password: String,
}

#[derive(Deserialize, ToSchema, Validate, Debug, Clone)]
///
pub struct PasswordChangeRequest {
    ///
    #[garde(length(min = 8, max = 100), custom(validate_password))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub current_password: String,

    ///
    #[garde(length(min = 8, max = 100), custom(validate_password))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub new_password: String,

    ///
    #[garde(
        length(min = 8, max = 100),
        matches(new_password),
        custom(validate_password)
    )]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub new_password_repeat: String,
}
