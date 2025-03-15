use repository::common::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Invalid password")]
    InvalidPassword,

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("Hasher error: {0}")]
    Hasher(#[from] utils::auth::password_hashing::Error),

    #[error("{0}")]
    Database(String),
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        use RepositoryError as RE;
        match err {
            #[cfg(feature = "surrealdb")]
            RE::SurrealDB(..) => Self::Database(err.to_string()),
        }
    }
}
