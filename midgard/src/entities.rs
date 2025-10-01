use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub identifier: Uuid,
    pub currency_code: String,
    pub currency: String,
    pub country: String,
    pub flag: Option<String>,
}
