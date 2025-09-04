use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

use crate::{authentication::service::AuthenticationService, users::service::UsersService};

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
    pub fn init(pool: Arc<Pool<Postgres>>) -> Self {
        let users_service = UsersService::init(pool);
        Self {
            authentication_service: AuthenticationService::init(users_service.clone()),
            users_service,
        }
    }
}
