use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct DatabaseInsertResult {
    pub identifier: Uuid,
}
