use crate::errors::RepositoryError;
use crate::shared::repository::DatabaseInsertResult;
use crate::wallet::adapters::CreateWalletRequest;
use crate::wallet::entities::{PaginatedWallet, Wallet};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct WalletRepository {
    pub pool: PgPool,
}

impl WalletRepository {
    pub fn new(pool: PgPool) -> Self {
        WalletRepository { pool }
    }
}

pub trait WalletRepositoryExt {
    fn create(
        &self,
        payload: &CreateWalletRequest,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Uuid, RepositoryError>> + Send;

    fn fetch_wallet(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Wallet>, RepositoryError>> + Send;

    fn fetch_all_wallet(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<PaginatedWallet, RepositoryError>> + Send;
}

impl WalletRepositoryExt for WalletRepository {
    async fn create(
        &self,
        payload: &CreateWalletRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, RepositoryError> {
        let query = r#"
        INSERT INTO wallets (identifier, name, user_identifier, country_identifier) VALUES ($1, $2, $3, $4) RETURNING identifier;"#;
        let insert_result = sqlx::query_as::<_, DatabaseInsertResult>(query)
            .bind(Uuid::new_v4())
            .bind(&payload.name)
            .bind(user_identifier)
            .bind(&payload.currency_identifier)
            .fetch_one(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

        Ok(insert_result.identifier)
    }

    async fn fetch_all_wallet(
        &self,
        user_identifier: &Uuid,
    ) -> Result<PaginatedWallet, RepositoryError> {
        todo!()
    }

    async fn fetch_wallet(
        &self,
        identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<Wallet>, RepositoryError> {
        let query = r#"SELECT * from wallets WHERE identifier = $1 AND user_identifier = $2"#;

        sqlx::query_as::<_, Wallet>(query)
            .bind(identifier)
            .bind(user_identifier)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::from)
    }
}
