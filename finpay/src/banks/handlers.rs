use crate::banks::entities::Bank;
use crate::banks::service::{BankService, BankServiceExt};
use crate::errors::ServiceError;
use crate::utils::ApiResponse;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn fetch_all_banks(
    State(bank_service): State<BankService>,
) -> Result<ApiResponse<Vec<Bank>>, ServiceError> {
    let banks = bank_service.fetch_all().await?;

    Ok(ApiResponse::builder().data(banks).build())
}

pub async fn fetch_bank_by_identifier(
    State(bank_service): State<BankService>,
    bank_identifier: Option<Path<Uuid>>,
) -> Result<ApiResponse<Bank>, ServiceError> {
    let Some(Path(identifier)) = bank_identifier else {
        return Err(ServiceError::UnprocessableEntity(
            "badly formatted country identifier".to_string(),
        ));
    };

    let bank = bank_service.fetch_by_identifier(&identifier).await?;

    Ok(ApiResponse::builder().data(bank).build())
}
