use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};
use finpay_redis::RedisClientError;

use crate::errors::{AppError, AuthenticationError, RepositoryError};
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
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error("badly formatted request")]
    BadRequest,
    #[error("an internal error occured")]
    AppError(#[from] AppError),
    #[error("an internal error occured while parsing message")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("an internal error occured due to redis client")]
    RedisClientError(#[from] RedisClientError),
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
            ServiceError::AuthenticationError(err) => err.status_code(),
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::AppError(err) => err.status_code(),
            ServiceError::RedisClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::SerdeJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
