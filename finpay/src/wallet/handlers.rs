use crate::authentication::claims::Claims;
use crate::errors::ServiceError;
use crate::utils::{ApiResponse, AuthenticatedRequest, PaginatedResponse, PaginationParams};
use crate::wallet::adapters::CreateWalletRequest;
use crate::wallet::entities::Wallet;
use crate::wallet::service::{WalletService, WalletServiceExt};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_wallet(
    State(wallet_service): State<WalletService>,
    AuthenticatedRequest { claims, request }: AuthenticatedRequest<CreateWalletRequest>,
) -> Result<ApiResponse<Wallet>, ServiceError> {
    let inserted_identifier = wallet_service.create_wallet(&claims, &request).await?;
    let wallet = wallet_service
        .fetch_wallet(&claims, &inserted_identifier)
        .await?;

    Ok(ApiResponse::builder()
        .data(wallet)
        .status_code(StatusCode::CREATED)
        .build())
}

pub async fn fetch_wallet(
    State(wallet_service): State<WalletService>,
    claims: Claims,
    Path(wallet_identifier): Path<Uuid>,
) -> Result<ApiResponse<Wallet>, ServiceError> {
    let wallet = wallet_service
        .fetch_wallet(&claims, &wallet_identifier)
        .await?;

    Ok(ApiResponse::builder()
        .data(wallet)
        .status_code(StatusCode::CREATED)
        .build())
}

pub async fn fetch_all_wallets(
    State(wallet_service): State<WalletService>,
    claims: Claims,
    Query(pagination_params): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<Wallet>>, ServiceError> {
    let response = wallet_service
        .fetch_all_wallets(&claims, &pagination_params)
        .await?;
    Ok(ApiResponse::builder().data(response).build())
}

pub async fn retrieve_wallet_balance(
    State(wallet_service): State<WalletService>,
    claims: Claims,
    Path(wallet_identifier): Path<Uuid>,
) -> Result<ApiResponse<PaginatedResponse<Wallet>>, ServiceError> {
  todo!()
}
