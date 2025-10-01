use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::utils::ApiResponseBuilder;

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

impl RepositoryError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RepositoryError::RecordNotFound => StatusCode::NOT_FOUND,
            RepositoryError::DuplicateRecord => StatusCode::CONFLICT,
            RepositoryError::SqlxError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RepositoryError::OperationFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}
impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
