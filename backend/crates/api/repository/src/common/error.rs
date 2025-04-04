#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[cfg(feature = "surrealdb")]
    #[error("Database error: {0}")]
    SurrealDB(#[from] surrealdb::Error),

    #[cfg(feature = "surrealdb")]
    #[error("Failed to save object to the database")]
    FailedToSaveObject,

    #[cfg(feature = "s3")]
    #[error("S3 error: {0}")]
    S3(#[from] aws_sdk_s3::Error),

    #[cfg(feature = "s3")]
    #[error("Got broken image from S3")]
    S3BrokenImage,

    #[cfg(feature = "s3")]
    #[error("S3 error: {0}")]
    S3ByteStream(#[from] aws_sdk_s3::primitives::ByteStreamError),
}
