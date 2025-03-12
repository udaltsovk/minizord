use repository::common::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    Database(String),
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            #[cfg(feature = "surrealdb")]
            RepositoryError::SurrealDB(..) => Self::Database(err.to_string()),
        }
    }
}
