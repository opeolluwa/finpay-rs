use axum_extra::headers::UserAgent;
use axum_extra::headers::authorization::Bearer;
use axum_typed_multipart::TypedMultipart;
use bcrypt::hash;
use bcrypt::{DEFAULT_COST, verify};
use finpay_imagekit::ImagekitClient;
use finpay_mailer::{EmailClient, EmailClientExt};
use finpay_redis::{RedisClient, RedisClientExt};
use finpay_utils::{extract_env, generate_file_name};
use std::path::Path;
use std::time::Duration;
use uuid::Uuid;

use crate::authentication::adapter::{CreateUserResponse, VerifyResetOtpResponse};
use crate::authentication::adapter::{
    ForgottenPasswordRequest, ForgottenPasswordResponse, LoginResponse, RefreshTokenResponse,
    SetNewPasswordRequest, SetNewPasswordResponse, VerifyAccountResponse, VerifyOtpRequest,
};
use crate::authentication::adapter::{LoginRequest, UploadProfilePictureRequest};
use crate::authentication::claims::{Claims, TWENTY_FIVE_MINUTES};
use crate::config::AppConfig;
use crate::errors::AuthenticationError::{InvalidOtp, Unauthenticated};
use crate::errors::RepositoryError::DuplicateRecord;
use crate::otp::service::{OtpService, OtpServiceExt};
use crate::users::entities::User;
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
        request: &VerifyOtpRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn validate_otp(
        &self,
        claims: &Claims,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<User, ServiceError>> + Send;

    fn request_refresh_token(
        &self,
        request: &Bearer,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, ServiceError>> + Send;

    fn set_avatar_url(
        &self,
        request: TypedMultipart<UploadProfilePictureRequest>,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyOtpRequest,
    ) -> impl std::future::Future<Output = Result<VerifyResetOtpResponse, ServiceError>> + Send;

    fn blacklist_token(
        &self,
        bearer: &Bearer,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn authorize(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<LoginResponse, ServiceError>> + Send;
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
        let user_identifier = self.user_service.create_account(payload).await?;

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

    async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        let user = self.user_service.find_user_by_email(&request.email).await?;

        let is_valid_password = self.validate_password(&request.password, &user.password)?;
        if !is_valid_password {
            return Err(ServiceError::AuthenticationError(WrongCredentials));
        }

        let auth = self.authorize(&user.identifier).await?;
        Ok(auth)
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
        _user_agent: &UserAgent, //TODO: add user agen tto message
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
                .send_forgotten_password_email(&email, &otp, &first_name)
                .await
            {
                log::error!("Failed to send account password reset email: {error}");
            }
        });

        Ok(ForgottenPasswordResponse { token })
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, ServiceError> {
        let hash = self.hash_password(&request.password)?;
        self.user_service
            .set_password(&claims.user_identifier, &hash)
            .await?;

        Ok(SetNewPasswordResponse {})
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyOtpRequest,
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

    async fn validate_otp(&self, claims: &Claims, otp: &str) -> Result<User, ServiceError> {
        let user = self
            .user_service
            .find_user_by_pk(&claims.user_identifier)
            .await?;

        let is_valid_otp = self
            .otp_service
            .validate_otp_for_user(&claims.user_identifier, otp)
            .await?;

        if !is_valid_otp {
            return Err(ServiceError::AuthenticationError(InvalidOtp));
        }

        Ok(user)
    }

    async fn request_refresh_token(
        &self,
        bearer_token: &Bearer,
    ) -> Result<RefreshTokenResponse, ServiceError> {
        let refresh_token = bearer_token.token();
        let mut redis_client = RedisClient::new().await?;

        if redis_client
            .check_blacklisted_token(refresh_token)
            .await?
            .map(|token| !token.is_empty())
            .unwrap_or(false)
        {
            return Err(ServiceError::AuthenticationError(Unauthenticated));
        };

        let claims = Claims::from_token(refresh_token)?;
        let user = self
            .user_service
            .find_user_by_pk(&claims.user_identifier)
            .await?;

        let auth = self.authorize(&user.identifier).await?;
        Ok(auth)
    }

    async fn set_avatar_url(
        &self,
        TypedMultipart(UploadProfilePictureRequest { image }): TypedMultipart<
            UploadProfilePictureRequest,
        >,
        user_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        let file_name = image
            .metadata
            .file_name
            .clone()
            .unwrap_or(generate_file_name());

        let config = AppConfig::from_env()?;
        let temp_dir = Path::new(&config.upload_path);
        let file_path = temp_dir.join(format!(
            "{time_stamp}_{file_name}",
            time_stamp = chrono::Local::now().timestamp()
        ));

        // create file object
        if let Err(err) = image.contents.persist(&file_path) {
            log::error!("error processing file due to {err}");
            return Err(ServiceError::OperationFailed);
        }

        let private_key = extract_env::<String>("IMAGEKIT_PRIVATE_KEY");
        let public_key = extract_env::<String>("IMAGEKIT_PUBLIC_KEY");

        let imagekit_upload_response = ImagekitClient::new(&public_key, &private_key)
            .map_err(|err| {
                log::error!("error creating client due to {err}");
                ServiceError::OperationFailed
            })?
            .upload_file(&file_path, &file_name)
            .await
            .map_err(|err| {
                log::error!("error creating client due to {err}");
                ServiceError::OperationFailed
            })?;

        let avatar_url = imagekit_upload_response.url;
        self.user_service
            .set_avatar_url(user_identifier, &avatar_url)
            .await?;

        Ok(())
    }

    async fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyOtpRequest,
    ) -> Result<VerifyResetOtpResponse, ServiceError> {
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

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        Ok(VerifyResetOtpResponse { token })
    }

    async fn blacklist_token(&self, bearer: &Bearer) -> Result<(), ServiceError> {
        let token = bearer.token();
        let mut redis_client = RedisClient::new().await?;
        redis_client.blacklist_refresh_token(token).await?;
        Ok(())
    }

    async fn authorize(&self, user_identifier: &Uuid) -> Result<LoginResponse, ServiceError> {
        let user = self.user_service.find_user_by_pk(user_identifier).await?;

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
}
