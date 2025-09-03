use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Freelancer,
    Company,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Freelancer => write!(f, "{}", "Freelancer"),
            AccountType::Company => write!(f, "{}", "Company"),
        }
    }
}