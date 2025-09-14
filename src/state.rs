use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::countries::service::CountryService;
use crate::otp::service::OtpService;
use crate::{authentication::service::AuthenticationService, users::service::UsersService};

#[derive(Clone)]
pub struct AppState {
    users_service: UsersService,
    authentication_service: AuthenticationService,
    country_service: CountryService,
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

impl AppState {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let users_service = UsersService::new(&pool);
        let otp_service = OtpService::new(&pool);
        let authentication_service =
            AuthenticationService::new(users_service.clone(), otp_service.clone());
        let country_service = CountryService::new(&pool);
        Self {
            authentication_service,
            users_service,
            country_service,
        }
    }
}
