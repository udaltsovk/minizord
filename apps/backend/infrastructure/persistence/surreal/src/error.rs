type DBError = surrealdb::Error;

#[derive(thiserror::Error, Debug)]
pub enum SurrealAdapterError {
    #[error(transparent)]
    Pool(#[from] mobc::Error<DBError>),

    #[error(transparent)]
    Database(#[from] DBError),
}
