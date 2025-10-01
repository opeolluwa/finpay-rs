use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "forgotten_password.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForgottenPasswordTemplate {
    pub email: String,
    pub otp: String,
    pub first_name: String,
}

impl ForgottenPasswordTemplate {
    pub fn new(otp: &str, email: &str, first_name: &str) -> Self {
        Self {
            otp: otp.to_string(),
            email: email.to_string(),
            first_name: first_name.to_string(),
        }
    }
}
