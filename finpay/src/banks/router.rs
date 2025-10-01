use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use crate::banks::handlers::{fetch_all_banks, fetch_bank_by_identifier};

pub fn banks_routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(fetch_all_banks))
        .route("/{bank_identifier}", get(fetch_bank_by_identifier))
        .with_state(state.clone())
}
