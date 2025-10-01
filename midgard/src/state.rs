use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::service::CountryService;

#[derive(Clone)]
pub struct AppState {
    country_service: CountryService,
}

impl FromRef<AppState> for CountryService {
    fn from_ref(services: &AppState) -> CountryService {
        services.country_service.clone()
    }
}

impl AppState {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let country_service = CountryService::new(&pool);

        Self { country_service }
    }
}
