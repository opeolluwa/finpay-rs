use finpay_mailer::ConfirmEmailTemplate;
use finpay_mailer::Email;
use finpay_mailer::EmailClient;
use finpay_mailer::ForgottenPasswordTemplate;
use finpay_mailer::PasswordUpdatedTemplate;
use finpay_mailer::WelcomeTemplate;

use crate::authentication::claims::{Claims, TWENTY_FIVE_MINUTES};
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
            .from("admin@eckko.oapp")
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
            .from("admin@eckko.app")
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
}

pub trait AuthenticationServiceExt {
    fn register(
        &self,
        payload: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<User, ServiceError>> + Send;
    fn login(
        &self,
        payload: &LoginUserRequest,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn logout(&self) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn forgot_password(&self)
    -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn reset_password(&self) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AuthenticationServiceExt for AuthenticationService {
    async fn register(&self, payload: &CreateUserRequest) -> Result<User, ServiceError> {
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

        Ok(user)
    }

    async fn login(&self, payload: &LoginUserRequest) -> Result<(), ServiceError> {
        todo!()
    }

    async fn logout(&self) -> Result<(), ServiceError> {
        todo!()
    }

    async fn forgot_password(&self) -> Result<(), ServiceError> {
        todo!()
    }

    async fn reset_password(&self) -> Result<(), ServiceError> {
        todo!()
    }
}
