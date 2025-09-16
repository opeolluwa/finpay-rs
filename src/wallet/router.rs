use crate::{
    state::AppState,
    wallet::handlers::{create_wallet, fetch_all_wallets, fetch_wallet},
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn wallet_routes(state: &AppState) -> Router {
    Router::new()
        .route("/", post(create_wallet))
        .route("/{wallet_identifier}", get(fetch_wallet))
        .route("/", get(fetch_all_wallets))
        .with_state(state.clone())
}
