use std::fmt;

#[derive(Debug)]
pub enum ImagekitError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    UploadFailed(String),
}

impl fmt::Display for ImagekitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImagekitError::Reqwest(err) => write!(f, "Request error: {}", err),
            ImagekitError::Io(err) => write!(f, "I/O error: {}", err),
            ImagekitError::InvalidHeader(err) => write!(f, "Invalid header: {}", err),
            ImagekitError::UploadFailed(msg) => write!(f, "Upload failed: {}", msg),
        }
    }
}

impl std::error::Error for ImagekitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ImagekitError::Reqwest(err) => Some(err),
            ImagekitError::Io(err) => Some(err),
            ImagekitError::InvalidHeader(err) => Some(err),
            ImagekitError::UploadFailed(_) => None,
        }
    }
}

impl From<reqwest::Error> for ImagekitError {
    fn from(err: reqwest::Error) -> Self {
        ImagekitError::Reqwest(err)
    }
}

impl From<std::io::Error> for ImagekitError {
    fn from(err: std::io::Error) -> Self {
        ImagekitError::Io(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ImagekitError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        ImagekitError::InvalidHeader(err)
    }
}
