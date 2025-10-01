use crate::entities::Country;
use crate::errors::RepositoryError;
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]

pub struct CountryRepository {
    pool: PgPool,
}

impl CountryRepository {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        CountryRepository { pool: pool.clone() }
    }
}
pub trait CountryRepositoryExt {
    fn fetch_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Country>, RepositoryError>> + Send;

    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Country, RepositoryError>> + Send;
}

impl CountryRepositoryExt for CountryRepository {
    async fn fetch_all(&self) -> Result<Vec<Country>, RepositoryError> {
        let query = r"SELECT * FROM countries;";

        let result = sqlx::query_as::<_, Country>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Result<Country, RepositoryError> {
        let query = r"SELECT * FROM countries WHERE identifier = $1;";
        sqlx::query_as::<_, Country>(query)
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(RepositoryError::RecordNotFound)
    }
}
