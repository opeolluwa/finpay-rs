use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateOtpRequest {
    pub user_identifier: Uuid,
    pub token: String,
}

impl GenerateOtpRequest {
    pub fn new(user_identifier: Uuid, otp: &str) -> Self {
        Self {
            user_identifier,
            token: otp.to_owned(),
        }
    }
}
