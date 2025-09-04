use std::fmt::Debug;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    data: Option<T>,
    #[serde(skip)]
    status_code: StatusCode,
}

impl From<ServiceError> for ApiResponse<()> {
    fn from(value: ServiceError) -> Self {
        ApiResponse {
            message: None,
            data: Some(()),
            status_code: value.into_response().status(),
        }
    }
}

pub type EmptyResponseBody = ();

#[derive(Debug)]
pub struct ApiResponseBuilder<T: Serialize> {
    status_code: StatusCode,
    message: Option<String>,
    data: Option<T>,
}

impl<T> Default for ApiResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status_code: StatusCode::OK,
            message: None,
            data: None,
        }
    }
}

impl<T> ApiResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            message: self.message,
            data: self.data,
            status_code: self.status_code,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        #[derive(Debug, Serialize)]
        struct BaseResponse<T> {
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<T>,
        }
        let body = Json(BaseResponse {
            message: self.message,
            data: self.data,
        });
        (self.status_code, body).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn builder() -> ApiResponseBuilder<T> {
        ApiResponseBuilder::new()
    }
}
