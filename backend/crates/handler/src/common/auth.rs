use macros::request;

request![
    Login {
        login: String,
        password: String
    },
    PasswordChange {
        current_password: String,
        new_password: String,
        new_password_repeat: String,
    }
];
