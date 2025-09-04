use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{FromRequest, Request};
use axum::{Form, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::ServiceError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServiceError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(ServiceError::AxumJsonRejection)?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
