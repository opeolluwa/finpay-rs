use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::authentication::service::AuthenticationService;
use crate::security::otp::service::OtpService;
use crate::users::service::UsersService;

#[derive(Clone)]
pub struct AppState {
    users_service: UsersService,
    authentication_service: AuthenticationService,
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

impl AppState {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        let users_service = UsersService::new(&pool);
        let otp_service = OtpService::new(&pool);
        let authentication_service =
            AuthenticationService::new(users_service.clone(), otp_service.clone());

        Self {
            authentication_service,
            users_service,
        }
    }
}
