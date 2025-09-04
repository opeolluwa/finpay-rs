use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub fn users_router(state: AppState) -> Router {
    Router::new().with_state(state)
}
