use crate::authentication::claims::Claims;
use crate::errors::RepositoryError::RecordNotFound;
use crate::errors::ServiceError;
use crate::errors::ServiceError::RepositoryError;
use crate::utils::{PaginatedResponse, PaginationParams};
use crate::wallet::adapters::CreateWalletRequest;
use crate::wallet::entities::Wallet;
use crate::wallet::repository::{WalletRepository, WalletRepositoryExt};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct WalletService {
    repository: WalletRepository,
}

impl WalletService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repository: WalletRepository::new(pool.clone()),
        }
    }
}

pub trait WalletServiceExt: Send + Sync + 'static {
    fn create_wallet(
        &self,
        claims: &Claims,
        request: &CreateWalletRequest,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>> + Send;

    fn fetch_wallet(
        &self,
        claims: &Claims,
        wallet_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Wallet, ServiceError>> + Send;

    fn fetch_all_wallets(
        &self,
        claims: &Claims,
        pagination_params: &PaginationParams,
    ) -> impl std::future::Future<Output = Result<PaginatedResponse<Wallet>, ServiceError>> + Send;
}

impl WalletServiceExt for WalletService {
    async fn create_wallet(
        &self,
        claims: &Claims,
        request: &CreateWalletRequest,
    ) -> Result<Uuid, ServiceError> {
        self.repository
            .create(request, &claims.user_identifier)
            .await
            .map_err(ServiceError::from)
    }

    async fn fetch_wallet(
        &self,
        claims: &Claims,
        wallet_identifier: &Uuid,
    ) -> Result<Wallet, ServiceError> {
        self.repository
            .fetch_wallet(wallet_identifier, &claims.user_identifier)
            .await?
            .ok_or(RepositoryError(RecordNotFound))
    }

    async fn fetch_all_wallets(
        &self,
        claims: &Claims,
        pagination_params: &PaginationParams,
    ) -> Result<PaginatedResponse<Wallet>, ServiceError> {
        let result = self
            .repository
            .fetch_all_wallet(&claims.user_identifier, pagination_params)
            .await?;
        Ok(result)
    }
}
