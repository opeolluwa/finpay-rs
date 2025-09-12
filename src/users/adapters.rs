use fake::Dummy;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::users::enums::AccountType;

#[derive(Debug, Serialize, Deserialize, Clone, Validate, Dummy, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
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

#[derive(Debug, Serialize, Deserialize)]

pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}
