use axum::{http::StatusCode, response::IntoResponse};

use crate::utils::{ApiResponseBuilder, EmptyResponseBody};

#[derive(thiserror::Error, Debug)]
pub enum ExtractorError {
    #[error("Invalid source type: values not safely converted")]
    ConvertionError,
}

impl ExtractorError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::ConvertionError => StatusCode::BAD_REQUEST,
        }
    }
}
impl IntoResponse for ExtractorError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<EmptyResponseBody>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
