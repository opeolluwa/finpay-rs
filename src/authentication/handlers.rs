use axum::{extract::State, Json};

use crate::{
    authentication::service::{AuthenticationService, AuthenticationServiceExt},
    errors::ServiceError,
    shared::middlewares::validator::ValidatedRequest,
    users::{adapters::CreateUserRequest, entities::User},
    utils::ApiResponse,
};

pub async fn signup(
    State(service): State<AuthenticationService>,
    Json(payload): Json<CreateUserRequest>,

) -> Result<ApiResponse<User>, ServiceError> {
    let user = service.register(&payload).await?;

    Ok(ApiResponse::builder().data(user).build())
}

pub async fn login() {}
