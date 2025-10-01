use crate::accounts::adapters::AccountCreationParams;
use crate::errors::ServiceError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AccountRepository {
    pub pool: PgPool,
}

impl AccountRepository {
    pub fn new(pool: &PgPool) -> AccountRepository {
        Self { pool: pool.clone() }
    }
}

pub trait AccountRepositoryExt {
    async fn create_account(
        &self,
        params: &AccountCreationParams,
    ) -> Result<Uuid, ServiceError>

    async fn fetch_account(
        &self,
        account_identifier: &Uuid,
    ) -> Result<Uuid, ServiceError>;
}


impl AccountRepositoryExt for AccountRepository {
    async fn create_account(&self, params: &AccountCreationParams) -> Result<Uuid, ServiceError> {
        todo!()
    }

    async fn fetch_account(&self, account_identifier: &Uuid) -> Result<Uuid, ServiceError> {
        todo!()
    }
}