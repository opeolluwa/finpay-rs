use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "confirm_email.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfirmEmailTemplate {
    pub email: String,
    pub otp: String,
}

impl ConfirmEmailTemplate {
    pub fn new(email: &str, otp: &str) -> Self {
        Self {
            email: email.to_string(),
            otp: otp.to_string(),
        }
    }
}
