use crate::errors::ServiceError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(transparent)]
pub struct FlattenedOption<T>(Option<T>);

impl<T> Serialize for FlattenedOption<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Some(inner_value) => inner_value.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl<T> Deref for FlattenedOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Option<T>> for FlattenedOption<T> {
    fn from(value: Option<T>) -> Self {
        FlattenedOption(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(flatten)]
    data: FlattenedOption<T>,

    #[serde(skip)]
    status_code: StatusCode,
}

impl From<ServiceError> for ApiResponse<()> {
    fn from(value: ServiceError) -> Self {
        ApiResponse {
            message: None,
            data: FlattenedOption::default(),
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
            data: self.data.into(),
            status_code: self.status_code,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn builder() -> ApiResponseBuilder<T> {
        ApiResponseBuilder::new()
    }

    pub fn success(data: T) -> ApiResponse<T> {
        ApiResponse {
            data: FlattenedOption(Some(data)),
            status_code: StatusCode::OK,
            message: None,
        }
    }

    pub fn error<S: Into<String>>(status_code: StatusCode, message: S) -> ApiResponse<()> {
        ApiResponse {
            status_code,
            message: Some(message.into()),
            data: FlattenedOption::default(),
        }
    }
}
