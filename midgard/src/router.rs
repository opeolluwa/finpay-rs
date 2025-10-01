use crate::handlers::{fetch_all_countries, fetch_country_by_identifier};
use crate::{
    state::AppState,
    utils::{ApiResponseBuilder, EmptyResponseBody},
};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use sqlx::Pool;
use std::sync::Arc;

fn country_routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(fetch_all_countries))
        .route("/{country_identifier}", get(fetch_country_by_identifier))
        .with_state(state.clone())
}

pub fn load_routes(pool: Arc<Pool<sqlx::Postgres>>) -> Router {
    let router = Router::new();

    let state = AppState::new(pool);

    router
        .nest("/countries", country_routes(&state))
        .route("/health", get(async move || "Healthy..."))

        .fallback(async || {
            ApiResponseBuilder::<EmptyResponseBody>::new()
                .message(
                    "The resource you're looking for does not exist, or it has been permanently moved.",
                )
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        })
}
