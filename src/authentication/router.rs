use axum::routing::get;
use axum::{Router, routing::post};

use crate::authentication::handlers::{
    forgotten_password, login, request_refresh_token, set_new_password, verify_account,
};
use crate::{authentication::handlers::signup, state::AppState};

pub fn authentication_routers(state: &AppState) -> Router {
    Router::new()
        .route("/register", post(signup))
        .route("/verify", post(verify_account))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/reset-password", post(set_new_password))
        .route("/verify-account", post(verify_account))
        .route("/refresh-token", get(request_refresh_token))
        .with_state(state.clone())
}
