use crate::banks::entities::Bank;
use crate::banks::repository::{BankRepository, BankRepositoryExt};
use crate::errors::ServiceError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct BankService {
    repository: BankRepository,
}

impl BankService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repository: BankRepository::new(pool),
        }
    }
}

pub trait BankServiceExt {
    fn fetch_all(&self) -> impl Future<Output = Result<Vec<Bank>, ServiceError>> + Send;

    fn fetch_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl Future<Output = Result<Bank, ServiceError>> + Send;

    fn fetch_local_operating_banks(
        &self,
        country_identifier: &Uuid,
    ) -> impl Future<Output = Result<Vec<Bank>, ServiceError>> + Send;
}

impl BankServiceExt for BankService {
    async fn fetch_all(&self) -> Result<Vec<Bank>, ServiceError> {
        let banks = self.repository.fetch_all().await?;
        Ok(banks)
    }

    async fn fetch_by_identifier(&self, identifier: &Uuid) -> Result<Bank, ServiceError> {
        let bank = self.repository.find_by_identifier(identifier).await?;
        Ok(bank)
    }

    async fn fetch_local_operating_banks(
        &self,
        identifier: &Uuid,
    ) -> Result<Vec<Bank>, ServiceError> {
        let banks = self
            .repository
            .find_by_country_identifier(identifier)
            .await?;
        Ok(banks)
    }
}
