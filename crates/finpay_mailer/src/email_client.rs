use askama::Template;
use lettre::{
    SmtpTransport, Transport,
    message::{Mailbox, MultiPart, SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use serde::Serialize;

use finpay_utils::extract_env;

use crate::{
    ConfirmEmailTemplate, ForgottenPasswordTemplate, PasswordUpdatedTemplate, WelcomeTemplate,
    email::Email, errors::EmailError,
};

#[derive(Debug, Clone)]
pub struct EmailClient {
    mailer: SmtpTransport,
}

impl EmailClient {
    pub fn new() -> Self {
        let smtp_host: String = extract_env("SMTP_HOST");
        let smtp_port: u16 = extract_env("SMTP_PORT");
        let smtp_username: String = extract_env("SMTP_AUTH_USERNAME");
        let smtp_password: String = extract_env("SMTP_AUTH_PASSWORD");
        let environment = extract_env::<String>("ENVIRONMENT");

        if environment == "prod" {
            let creds = Credentials::new(smtp_username, smtp_password);
            let mailer = SmtpTransport::relay(&smtp_host)
                .expect("Failed to create SMTP relay")
                .port(smtp_port)
                .credentials(creds)
                .build();
            EmailClient { mailer }
        } else {
            let mailer = SmtpTransport::builder_dangerous("maildev")
                .port(1025)
                .build();
            EmailClient { mailer }
        }
    }

    pub fn send_email<T>(&self, email: &Email<T>) -> Result<(), EmailError>
    where
        T: Template + Send + Serialize + Default,
    {
        let Email {
            to,
            template,
            subject,
            from,
            ..
            // _reply_to,
        } = email;

        let email_content = template
            .render()
            .map_err(|e| EmailError::TemplateError(e.to_string()))?;

        let email: Mailbox = email
            .from
            .parse()
            .map_err(|_| EmailError::InvalidEmail(from.clone()))?;

        let to: Mailbox = to
            .parse()
            .map_err(|_| EmailError::InvalidEmail(to.clone()))?;

        let message = lettre::Message::builder()
            .from(email)
            .to(to)
            .subject(subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(email_content),
                ),
            )
            .map_err(|e| {
                log::info!("failed to send email due to {e}");
                EmailError::SendError(e.to_string())
            })?;

        self.mailer.send(&message).map_err(|e| {
            log::info!("failed to send email due to {e}");
            EmailError::SendError(e.to_string())
        })?;
        Ok(())
    }

    pub fn test_connection(&self) -> Result<bool, EmailError> {
        self.mailer.test_connection().map_err(|err| {
            log::info!("failed to send email due to {err}");
            EmailError::SendError(err.to_string())
        })
    }
}

pub trait EmailClientExt {
    fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
        first_name: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send;

    fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send;

    fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send;

    fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send;
}

impl EmailClientExt for EmailClient {
    async fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
        first_name: &str,
    ) -> Result<(), EmailError> {
        let template = ConfirmEmailTemplate::new(user_email, otp, first_name);
        let email = Email::builder()
            .subject("Confirm your account")
            .to(user_email)
            .template(template)
            .build();

        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send confirmation email due to: {err}");
            EmailError::SendError(err.to_string())
        })?;

        Ok(())
    }

    async fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), EmailError> {
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
            EmailError::SendError(err.to_string())
        })?;

        Ok(())
    }
    async fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> Result<(), EmailError> {
        let email = Email::builder()
            .subject("Password Updated")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            EmailError::SendError(err.to_string())
        })?;

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            EmailError::SendError(err.to_string())
        })?;

        Ok(())
    }

    async fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> Result<(), EmailError> {
        let template = WelcomeTemplate::new(user_name);
        let email = Email::builder()
            .subject("Welcome to Finpay!")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send welcome email due to: {err}");
            EmailError::SendError(err.to_string())
        })?;

        Ok(())
    }
}
