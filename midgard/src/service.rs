use crate::entities::Country;
use crate::errors::ServiceError;
use crate::repository::{CountryRepository, CountryRepositoryExt};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct CountryService {
    repository: CountryRepository,
}

impl CountryService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repository: CountryRepository::new(pool),
        }
    }
}

pub trait CountryServiceExt {
    fn fetch_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Country>, ServiceError>> + Send;

    fn fetch_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Country, ServiceError>> + Send;
}

impl CountryServiceExt for CountryService {
    async fn fetch_all(&self) -> Result<Vec<Country>, ServiceError> {
        let countries = self.repository.fetch_all().await?;
        Ok(countries)
    }

    async fn fetch_by_identifier(&self, identifier: &Uuid) -> Result<Country, ServiceError> {
        let country = self.repository.find_by_identifier(identifier).await?;
        Ok(country)
    }
}
