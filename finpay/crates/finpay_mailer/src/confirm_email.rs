use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "confirm_email.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfirmEmailTemplate {
    pub email: String,
    pub otp: String,
    pub first_name: String,
}

impl ConfirmEmailTemplate {
    pub fn new(email: &str, otp: &str, first_name: &str) -> Self {
        Self {
            email: email.to_string(),
            otp: otp.to_string(),
            first_name: first_name.to_string(),
        }
    }
}
