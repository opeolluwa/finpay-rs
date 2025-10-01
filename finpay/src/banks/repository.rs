use crate::banks::entities::Bank;
use crate::errors::RepositoryError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct BankRepository {
    pool: PgPool,
}

impl BankRepository {
    pub fn new(pool: &PgPool) -> BankRepository {
        BankRepository { pool: pool.clone() }
    }
}

pub trait BankRepositoryExt {
    fn fetch_all(
        &self,
    ) -> impl Future<Output=Result<Vec<Bank>, RepositoryError>> + Send;

    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl Future<Output = Result<Bank, RepositoryError> > + Send;

    fn find_by_country_identifier(
        &self,
        country_identifier: &Uuid,
    ) -> impl Future<Output=Result<Vec<Bank>, RepositoryError>> + Send;
}

impl BankRepositoryExt for BankRepository {
    async fn fetch_all(&self) -> Result<Vec<Bank>, RepositoryError> {
        let query = r"SELECT * FROM banks;";

        let result = sqlx::query_as::<_, Bank>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Result<Bank, RepositoryError> {
        let query = r"SELECT * FROM banks WHERE identifier = $1;";
        sqlx::query_as::<_, Bank>(query)
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepositoryError::RecordNotFound)
    }

    async fn find_by_country_identifier(
        &self,
        country_identifier: &Uuid,
    ) -> Result<Vec<Bank>, RepositoryError> {
        let query = r"SELECT * FROM countries WHERE country_identifier = $1;";
        let banks = sqlx::query_as::<_, Bank>(query)
            .bind(country_identifier)
            .fetch_all(&self.pool)
            .await?;

        Ok(banks)
    }
}
