use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Bank {
    pub identifier: Uuid,
    pub bank_name: String,
    pub country_identifier: Uuid,
    pub created_date: DateTime<Local>,
    pub updated_ad: Option<DateTime<Local>>,
}
