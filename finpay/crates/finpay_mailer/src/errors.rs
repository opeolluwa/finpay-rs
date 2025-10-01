#[derive(thiserror::Error, Debug)]
pub enum EmailError {
    #[error("Email sending failed: {0}")]
    SendError(String),
    #[error("Template rendering failed: {0}")]
    TemplateError(String),
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
}
