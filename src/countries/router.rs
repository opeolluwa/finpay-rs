use crate::countries::handlers::{fetch_all_countries, fetch_country_by_identifier};
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn country_routes(state: &AppState) -> Router {
    Router::new()
        .route("/", post(fetch_all_countries))
        .route("/{country_identifier}", get(fetch_country_by_identifier))
}
