use axum::{routing::post, Router};

use crate::{authentication::handlers::signup, state::AppState};
use crate::authentication::handlers::verify_account;

pub fn authentication_routers(state: &AppState) -> Router {
    Router::new()
        .route("/register", post(signup))
        .route("/verify", post(verify_account))
        .with_state(state.clone())
}
