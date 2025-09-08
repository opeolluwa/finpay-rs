use std::time::Duration;

use axum_extra::headers::UserAgent;
use bcrypt::hash;
use bcrypt::{DEFAULT_COST, verify};
use finpay_mailer::{EmailClient, EmailClientExt};
use finpay_redis::{RedisClient, RedisClientExt};
use uuid::Uuid;

use crate::authentication::adapter::CreateUserResponse;
use crate::authentication::adapter::LoginRequest;
use crate::authentication::adapter::{
    ChangePasswordRequest, ForgottenPasswordRequest, ForgottenPasswordResponse, LoginResponse,
    RefreshTokenRequest, RefreshTokenResponse, SetNewPasswordRequest, SetNewPasswordResponse,
    VerifyAccountRequest, VerifyAccountResponse,
};
use crate::authentication::claims::{Claims, TWENTY_FIVE_MINUTES};
use crate::errors::AuthenticationError::InvalidOtp;
use crate::errors::RepositoryError::DuplicateRecord;
use crate::otp::service::{OtpService, OtpServiceExt};
use crate::{
    errors::ServiceError,
    users::{
        adapters::CreateUserRequest,
        service::{UsersService, UsersServiceExt},
    },
};

use crate::errors::AuthenticationError::WrongCredentials;
#[derive(Clone)]
pub struct AuthenticationService {
    user_service: UsersService,
    otp_service: OtpService,
}

impl AuthenticationService {
    pub fn new(user_service: UsersService, otp_service: OtpService) -> Self {
        Self {
            user_service,
            otp_service,
        }
    }

    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError> {
        hash(raw_password.trim(), DEFAULT_COST).map_err(|err| {
            log::error!("operation failed due to {err}");
            ServiceError::OperationFailed
        })
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        verify(password.trim(), hash).map_err(|err| {
            log::error!("operation failed due to {err}");
            ServiceError::OperationFailed
        })
    }
}

pub trait AuthenticationServiceExt {
    fn register(
        &self,
        payload: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<CreateUserResponse, ServiceError>> + Send;

    fn login(
        &self,
        request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, ServiceError>> + Send;

    fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
        user_agent: &UserAgent,
    ) -> impl std::future::Future<Output = Result<ForgottenPasswordResponse, ServiceError>> + Send;

    fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<SetNewPasswordResponse, ServiceError>> + Send;

    fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn validate_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<String, ServiceError>> + Send;

    fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, ServiceError>> + Send;

    fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn change_password(
        &self,
        request: &ChangePasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AuthenticationServiceExt for AuthenticationService {
    async fn register(
        &self,
        payload: &CreateUserRequest,
    ) -> Result<CreateUserResponse, ServiceError> {
        if self
            .user_service
            .find_user_by_email(&payload.email)
            .await
            .ok()
            .is_some()
        {
            return Err(ServiceError::RepositoryError(DuplicateRecord));
        }
        let user_identifier = self.user_service.create_account(&payload).await?;

        let user = self.user_service.find_user_by_pk(&user_identifier).await?;

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user_identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        let otp = self.otp_service.new_otp_for_user(&user.identifier).await?;
        let first_name = user.first_name.clone();
        let email = user.email.clone();

        tokio::task::spawn(async move {
            if let Err(error) = EmailClient::new()
                .send_account_confirmation_email(&email, &otp, &first_name)
                .await
            {
                log::error!("Failed to send account confirmation email: {error}");
            }
        });

        Ok(CreateUserResponse { token })
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        let user = self
            .user_service
            .find_user_by_pk(&claims.user_identifier)
            .await?;

        let is_valid_otp = self
            .otp_service
            .validate_otp_for_user(&claims.user_identifier, &request.otp)
            .await?;

        if !is_valid_otp {
            return Err(ServiceError::AuthenticationError(InvalidOtp));
        }

        self.user_service
            .set_verified(&claims.user_identifier)
            .await?;

        let email_client = EmailClient::new();
        let user_email = user.email.clone();
        let user_name = user.first_name.clone();

        tokio::spawn(async move {
            if let Err(err) = email_client
                .send_welcome_email(&user_email, &user_name)
                .await
            {
                tracing::error!("Failed to send welcome email to {}: {}", user_email, err);
            }
        });

        Ok(VerifyAccountResponse {})
    }

    async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        let user = self.user_service.find_user_by_email(&request.email).await?;

        let is_valid_password = self.validate_password(&request.password, &user.password)?;
        if !is_valid_password {
            return Err(ServiceError::AuthenticationError(WrongCredentials));
        }
        let access_token = Claims::builder()
            .subject("access_token")
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(Duration::from_secs(15 * 60 /*15 minutes */))
            .build()?;

        let refresh_token = Claims::builder()
            .subject("refresh_token")
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(Duration::from_secs(7 * 60 * 60 /*7 hours */))
            .build()?;

        let refresh_token_out = refresh_token.generate_token()?;
        let mut redis_client = RedisClient::new().await?;
        redis_client.save_refresh_token(&refresh_token_out).await?;

        Ok(LoginResponse {
            access_token: access_token.generate_token()?,
            refresh_token: refresh_token_out,
            refresh_token_exp: refresh_token.exp,
            iat: access_token.iat,
            exp: access_token.exp,
            refresh_token_iat: refresh_token.iat,
        })
    }

    async fn change_password(
        &self,
        request: &ChangePasswordRequest,
        claims: &Claims,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
        user_agent: &UserAgent,
    ) -> Result<ForgottenPasswordResponse, ServiceError> {
        let user = self.user_service.find_user_by_email(&request.email).await?;

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        let otp = self.otp_service.new_otp_for_user(&user.identifier).await?;
        let first_name = user.first_name.clone();
        let email = user.email.clone();

        tokio::task::spawn(async move {
            if let Err(error) = EmailClient::new()
                .send_account_confirmation_email(&email, &otp, &first_name)
                .await
            {
                log::error!("Failed to send account password reset email: {error}");
            }
        });

        Ok(ForgottenPasswordResponse { token })
    }

    async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, ServiceError> {
        todo!()
    }

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, ServiceError> {
        todo!()
    }

    async fn validate_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<String, ServiceError> {
        todo!()
    }

    async fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        todo!()
    }
}
