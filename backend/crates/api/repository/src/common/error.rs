type DBError = surrealdb::Error;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Pool(#[from] mobc::Error<DBError>),

    #[error("Database error: {0}")]
    Database(#[from] DBError),

    #[error("Failed to save object to the database")]
    FailedToSaveObject,

    #[error("S3 error: {0}")]
    S3(#[from] aws_sdk_s3::Error),

    #[error("Got broken image from S3")]
    S3BrokenImage,

    #[error("S3 error: {0}")]
    S3ByteStream(#[from] aws_sdk_s3::primitives::ByteStreamError),
}
