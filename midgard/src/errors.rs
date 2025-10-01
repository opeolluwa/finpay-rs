use axum::extract::rejection::FormRejection;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::utils::ApiResponse;
use crate::utils::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] sqlx::error::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error("an internal error occurred")]
    OperationFailed,
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),

    #[error("badly formatted request")]
    BadRequest,
    #[error("an internal error occurred")]
    AppError(#[from] AppError),
    #[error("an internal error occurred while parsing message")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::OperationFailed => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::RepositoryError(err) => err.status_code(),
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::AppError(err) => err.status_code(),
            ServiceError::SerdeJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        ApiResponse::<()>::builder()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}

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
