use serde::{Deserialize, Serialize};

use crate::users::enums::AccountType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub account_type: AccountType,
    pub country: String,
    pub country_code: String,
    pub address: String,
    pub phone_number: String,
    pub occupation: Option<String>,
}
