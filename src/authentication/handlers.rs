use axum::extract::State;

use crate::{
    authentication::{
        adapter::CreateAccountRequest,
        service::{AuthenticationService, AuthenticationServiceExt},
    },
    errors::ServiceError,
    shared::middlewares::validator::ValidatedRequest,
    users::entities::User,
    utils::ApiResponse,
};

#[axum::debug_handler]
pub async fn signup(
    State(service): State<AuthenticationService>,
    ValidatedRequest(payload): ValidatedRequest<CreateAccountRequest>,
) -> Result<ApiResponse<User>, ServiceError> {
    let user = service.register(&payload.into()).await?;

    Ok(ApiResponse::builder().data(user).build())
}

pub async fn login() {}
