use crate::entities::Country;
use crate::errors::ServiceError;
use crate::service::{CountryService, CountryServiceExt};
use crate::utils::ApiResponse;
use axum::debug_handler;
use axum::extract::{Path, State};
use uuid::Uuid;

#[debug_handler]
pub async fn fetch_all_countries(
    State(country_service): State<CountryService>,
) -> Result<ApiResponse<Vec<Country>>, ServiceError> {
    let countries = country_service.fetch_all().await?;

    Ok(ApiResponse::builder().data(countries).build())
}

pub async fn fetch_country_by_identifier(
    State(country_service): State<CountryService>,
    country_identifier: Option<Path<Uuid>>,
) -> Result<ApiResponse<Country>, ServiceError> {
    let Some(Path(identifier)) = country_identifier else {
        return Err(ServiceError::UnprocessableEntity(
            "badly formatted country identifier".to_string(),
        ));
    };

    let country = country_service.fetch_by_identifier(&identifier).await?;

    Ok(ApiResponse::builder().data(country).build())
}
