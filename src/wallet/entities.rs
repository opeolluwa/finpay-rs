use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Wallet {
    pub identifier: Uuid,
    pub name: String,
    pub balance: f64,
    pub user_identifier: Uuid,
    pub currency_identifier: Uuid,
}

pub struct PaginatedWallet {}
