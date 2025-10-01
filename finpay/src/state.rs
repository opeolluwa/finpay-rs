use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::authentication::service::AuthenticationService;
use crate::banks::service::BankService;
use crate::countries::service::CountryService;
use crate::security::otp::service::OtpService;
use crate::users::service::UsersService;
use crate::wallet::service::WalletService;

#[derive(Clone)]
pub struct AppState {
    users_service: UsersService,
    authentication_service: AuthenticationService,
    country_service: CountryService,
    wallet_service: WalletService,
    banks_service: BankService,
}

impl FromRef<AppState> for UsersService {
    fn from_ref(services: &AppState) -> UsersService {
        services.users_service.clone()
    }
}

impl FromRef<AppState> for AuthenticationService {
    fn from_ref(services: &AppState) -> AuthenticationService {
        services.authentication_service.clone()
    }
}

impl FromRef<AppState> for CountryService {
    fn from_ref(services: &AppState) -> CountryService {
        services.country_service.clone()
    }
}

impl FromRef<AppState> for WalletService {
    fn from_ref(services: &AppState) -> WalletService {
        services.wallet_service.clone()
    }
}

impl FromRef<AppState> for BankService {
    fn from_ref(services: &AppState) -> BankService {
        services.banks_service.clone()
    }
}

impl AppState {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let users_service = UsersService::new(&pool);
        let otp_service = OtpService::new(&pool);
        let authentication_service =
            AuthenticationService::new(users_service.clone(), otp_service.clone());
        let country_service = CountryService::new(&pool);
        let wallet_service = WalletService::new(&pool);
        let banks_service = BankService::new(&pool);

        Self {
            authentication_service,
            users_service,
            country_service,
            wallet_service,
            banks_service,
        }
    }
}
