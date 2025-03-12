#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[cfg(feature = "surrealdb")]
    #[error("Database error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
}
