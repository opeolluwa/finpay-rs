use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Error extracting env {0}")]
    EnvError(String),
}
