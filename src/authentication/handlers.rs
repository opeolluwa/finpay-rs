use axum::{extract::State, http::StatusCode};

use crate::authentication::adapter::{
    ChangePasswordRequest, ForgottenPasswordRequest, LoginResponse, RefreshTokenResponse,
    SetNewPasswordRequest, VerifyAccountRequest, VerifyAccountResponse,
};
use crate::authentication::claims::Claims;
use crate::utils::AuthenticatedRequest;
use crate::{
    authentication::{
        adapter::{CreateAccountRequest, CreateUserResponse, LoginRequest},
        service::{AuthenticationService, AuthenticationServiceExt},
    },
    errors::ServiceError,
    shared::middlewares::validator::ValidatedRequest,
    utils::ApiResponse,
};

pub async fn signup(
    State(service): State<AuthenticationService>,
    ValidatedRequest(payload): ValidatedRequest<CreateAccountRequest>,
) -> Result<ApiResponse<CreateUserResponse>, ServiceError> {
    let response = service.register(&payload.into()).await?;

    Ok(ApiResponse::builder()
        .message("account creation was successful")
        .data(response)
        .build())
}

pub async fn verify_account(
    State(authentication_service): State<AuthenticationService>,
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, ServiceError> {
    let verify_account_response = authentication_service
        .verify_account(&claims, &request)
        .await?;
    Ok(ApiResponse::builder()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .message("Account verified successfully")
        .build())
}

pub async fn login(
    State(authentication_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, ServiceError> {
    let login_response = authentication_service.login(&request).await?;
    Ok(ApiResponse::builder()
        .status_code(StatusCode::OK)
        .data(login_response)
        .message("logged in successfully")
        .build())
}

pub async fn forgotten_password(
    State(authentication_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<ForgottenPasswordRequest>, ServiceError> {
    let forgotten_password_response = authentication_service.forgotten_password(&request).await?;

    todo!()
}

pub async fn set_new_password(
    State(authentication_service): State<AuthenticationService>,
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<SetNewPasswordRequest>, // claims: Claims,
) -> Result<ApiResponse<()>, ServiceError> {
    let _ = authentication_service
        .set_new_password(&request, &claims)
        .await?;

    Ok(ApiResponse::builder()
        .data(())
        .message("password updated successfully")
        .build())
}

pub async fn request_refresh_token(
    State(authentication_service): State<AuthenticationService>,
    claims: Claims,
) -> Result<ApiResponse<RefreshTokenResponse>, ServiceError> {
    let refresh_token_response = authentication_service.request_refresh_token(&claims).await?;

    Ok(ApiResponse::builder()
        .data(refresh_token_response)
        .message("token updated successfully")
        .build())
}

pub async fn logout() -> Result<ApiResponse<()>, ServiceError> {
    todo!()
}

pub async fn verify_reset_otp(
    State(authentication_service): State<AuthenticationService>,
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, ServiceError> {
    let verify_account_response = authentication_service
        .verify_reset_otp(&claims, &request)
        .await?;
    Ok(ApiResponse::builder()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .message("OTP verified successfully")
        .build())
}

pub async fn change_password(
    State(user_service): State<AuthenticationService>,
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<ChangePasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    user_service.change_password(&request, &claims).await?;

    Ok(ApiResponse::builder()
        .message("User's password changed successfully")
        .build())
}
