use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "welcome.html")]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WelcomeTemplate {
    user_name: String,
}

impl WelcomeTemplate {
    pub fn new(user_name: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
        }
    }
}
