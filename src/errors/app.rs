#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
}
