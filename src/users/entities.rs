use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::users::enums::AccountType;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub identifier: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub account_type: AccountType,
    pub country: String,
    pub country_code: String,
    pub address: String,
    pub phone_number: String,
    pub occupation: Option<String>,
    pub created_date: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub is_verified: bool,
    #[serde(skip)]
    pub password: String,
}
