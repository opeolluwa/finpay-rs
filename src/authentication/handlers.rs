use axum::debug_handler;
use axum::{extract::State, http::StatusCode};
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;

use crate::authentication::adapter::{
    ForgottenPasswordRequest, ForgottenPasswordResponse, LoginResponse, RefreshTokenResponse,
    SetNewPasswordRequest, VerifyAccountResponse, VerifyOtpRequest, VerifyResetOtpResponse,
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
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<VerifyOtpRequest>,
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

#[debug_handler]
pub async fn forgotten_password(
    State(authentication_service): State<AuthenticationService>,
    user_agent: Option<TypedHeader<UserAgent>>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<ForgottenPasswordResponse>, ServiceError> {
    let TypedHeader(user_agent) = user_agent.ok_or_else(|| ServiceError::BadRequest)?;

    let forgotten_password_response = authentication_service
        .forgotten_password(&request, &user_agent)
        .await?;

    Ok(ApiResponse::builder()
        .data(forgotten_password_response)
        .message("request processed successfully")
        .build())
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
    let refresh_token_response = authentication_service
        .request_refresh_token(&claims)
        .await?;

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
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<VerifyOtpRequest>,
) -> Result<ApiResponse<VerifyResetOtpResponse>, ServiceError> {
    let verify_reset_otp_response = authentication_service
        .verify_reset_otp(&claims, &request)
        .await?;
    Ok(ApiResponse::builder()
        .status_code(StatusCode::OK)
        .data(verify_reset_otp_response)
        .message("OTP verified successfully")
        .build())
}

pub async fn change_password(
    State(user_service): State<AuthenticationService>,
    AuthenticatedRequest { request, claims }: AuthenticatedRequest<SetNewPasswordRequest>,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> Result<ApiResponse<()>, ServiceError> {
    if let Some(TypedHeader(user_agent)) = user_agent {
        // The client sent a user agent
        println!("{:#?}", user_agent);
    } else {
        // No user agent header
        todo!()
    }

    user_service.set_new_password(&request, &claims).await?;

    Ok(ApiResponse::builder()
        .message("User's password changed successfully")
        .build())
}
