use askama::Template;
use lettre::{
    SmtpTransport, Transport,
    message::{Mailbox, MultiPart, SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use serde::Serialize;

use finpay_utils::extract_env;

use crate::{email::Email, errors::EmailError};

pub struct EmailClient {
    mailer: SmtpTransport,
}

impl EmailClient {
    pub fn new() -> Self {
        let smtp_host: String = extract_env("SMTP_HOST")
            .unwrap_or_else(|_| panic!("SMTP_HOST environment variable not set"));
        let smtp_port: u16 = extract_env("SMTP_PORT")
            .unwrap_or_else(|_| panic!("SMTP_PORT environment variable not set"));
        let smtp_username: String = extract_env("SMTP_AUTH_USERNAME")
            .unwrap_or_else(|_| panic!("SMTP_AUTH_USERNAME environment variable not set"));
        let smtp_password: String = extract_env("SMTP_AUTH_PASSWORD")
            .unwrap_or_else(|_| panic!("SMTP_AUTH_PASSWORD environment variable not set"));

        let creds = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)
            .expect("Failed to create SMTP relay")
            .port(smtp_port)
            .credentials(creds)
            .build();

        EmailClient { mailer }
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
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
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
}

pub trait EmailClientExt {
    fn send_confirmation_email(
        &self,
        email: &Email<impl Template + Send + Serialize + Default>,
    ) -> Result<(), EmailError>;
}

impl EmailClientExt for EmailClient {
    fn send_confirmation_email(
        &self,
        email: &Email<impl Template + Send + Serialize + Default>,
    ) -> Result<(), EmailError> {
        self.send_email(email).map_err(|e| {
            log::error!("Failed to send confirmation email: {e}");
            e
        })
    }
}
