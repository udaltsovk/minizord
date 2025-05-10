use repository::common::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    BadRequest(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("{0}")]
    PayloadTooLarge(String),

    #[error(
        "expected file of one of the following mime types: {supported}, but got `{got}`"
    )]
    UnsupportedMediaType { supported: String, got: String },

    #[error("Hasher error: {0}")]
    Hasher(#[from] utils::auth::password_hashing::Error),

    #[error("{0}")]
    Database(String),

    #[error("{0}")]
    Internal(String),
}

impl From<RepositoryError> for ServiceError {
    #[tracing::instrument(skip_all, level = "trace")]
    fn from(err: RepositoryError) -> Self {
        #[allow(unused_imports)]
        use RepositoryError as RE;
        match err {
            #[cfg(feature = "surrealdb")]
            RE::SurrealDB(..) | RE::FailedToSaveObject => {
                Self::Database(err.to_string())
            },
            #[cfg(feature = "s3")]
            RE::S3(..) | RE::S3BrokenImage | RE::S3ByteStream(..) => {
                Self::Database(err.to_string())
            },
        }
    }
}
