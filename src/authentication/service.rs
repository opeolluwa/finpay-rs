use bcrypt::hash;
use bcrypt::{verify, DEFAULT_COST};
use finpay_mailer::ConfirmEmailTemplate;
use finpay_mailer::Email;
use finpay_mailer::EmailClient;
use finpay_mailer::ForgottenPasswordTemplate;
use finpay_mailer::PasswordUpdatedTemplate;
use finpay_mailer::WelcomeTemplate;
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
        adapters::{CreateUserRequest, LoginUserRequest},
        entities::User,
        service::{UsersService, UsersServiceExt},
    },
};

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

    async fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
        first_name: &str,
    ) -> Result<(), ServiceError> {
        let template = ConfirmEmailTemplate::new(user_email, otp, first_name);
        let email = Email::builder()
            .subject("Confirm your account")
            .to(user_email)
            .template(template)
            .build();

        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send confirmation email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError> {
        let template = ForgottenPasswordTemplate::new(otp, user_email);

        let email = Email::builder()
            .subject("Forgotten Password")
            .to(user_email)
            .template(template)
            .from("admin@finpay.app")
            .build();
        let email_client = EmailClient::new();

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send forgotten password email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }
    async fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> Result<(), ServiceError> {
        let email = Email::builder()
            .subject("Password Updated")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            ServiceError::OperationFailed
        })?;

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> Result<(), ServiceError> {
        let template = WelcomeTemplate::new(user_name);
        let email = Email::builder()
            .subject("Welcome to Finpay!")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send welcome email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
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

        let service = self.clone();

        tokio::task::spawn(async move {
            if let Err(error) = service
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
        self.user_service
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
        
        Ok(VerifyAccountResponse {})
    }

    async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        let user = self.user_service.find_user_by_email(&request.email).await?;

        todo!()
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
    ) -> Result<ForgottenPasswordResponse, ServiceError> {
        todo!()
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
