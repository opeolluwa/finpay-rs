use axum::{Router, routing::post};

use crate::{authentication::handlers::signup, state::AppState};

pub fn authentication_routers(state: AppState) -> Router {
    Router::new()
        .route("/register", post(signup))
        .with_state(state)
}
