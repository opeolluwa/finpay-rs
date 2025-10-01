use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "password_updated.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PasswordUpdatedTemplate {
    device: String,
    location: String,
    time: String,
    date: String,
    security_url: String,
}

impl PasswordUpdatedTemplate {
    pub fn new(device: &str, location: &str, time: &str, date: &str, security_url: &str) -> Self {
        Self {
            device: device.to_string(),
            location: location.to_string(),
            time: time.to_string(),
            date: date.to_string(),
            security_url: security_url.to_string(),
        }
    }
}
