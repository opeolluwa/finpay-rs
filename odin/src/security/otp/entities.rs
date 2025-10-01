use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Otp {
    pub identifier: Uuid,
    pub user_identifier: Uuid,
    pub token: String,
    pub created_date: DateTime<Local>,
}
