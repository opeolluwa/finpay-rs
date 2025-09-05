use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "forgotten_password.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForgottenPasswordTemplate {
    pub email: String,
    pub otp: String,
}

impl ForgottenPasswordTemplate {
    pub fn new(otp: &str, email: &str) -> Self {
        Self {
            otp: otp.to_string(),
            email: email.to_string(),
        }
    }
}
