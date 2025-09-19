use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub identifier: Uuid,
    pub name: String,
    pub balance: BigDecimal,
    pub user_identifier: Uuid,
    pub currency_identifier: Uuid,
    pub created_date: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, FromRow)]
pub struct WalletWithCount {
    pub id: Uuid,
    pub name: String,
    pub currency_identifier: Uuid,
    pub balance: bigdecimal::BigDecimal,
    pub created_date: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub total_count: i64, 
}

pub struct PaginatedWallet {}
