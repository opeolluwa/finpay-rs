use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub account_type: String,
    pub country: String,
    pub country_code: String,
    pub address: String,
    pub phone_number: String,
    pub occupation: Option<String>,
}
