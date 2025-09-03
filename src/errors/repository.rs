#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("The record does not exist or has been permanently removed")]
    RecordNotFound,
    #[error("The record you're trying to create already exists")]
    DuplicateRecord,
    #[error("Operation failed due to {0}")]
    OperationFailed(String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
