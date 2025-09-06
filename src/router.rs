use std::sync::Arc;

use crate::{
    authentication::router::authentication_routers,
    state::AppState,
    users::users_router,
    utils::{ApiResponseBuilder, EmptyResponseBody},
};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use sqlx::Pool;

pub fn load_routes(pool: Arc<Pool<sqlx::Postgres>>) -> Router {
    let router = Router::new();

    let state = AppState::new(pool);

    router
        .merge(users_router(&state))
        .nest("/auth", authentication_routers(&state))
        .route("/health", get(async move || "Healthy..."))
        .fallback(async || {
            ApiResponseBuilder::<EmptyResponseBody>::new()
                .message(
                    "the resource you're looking does not exist or it has been permanently moved",
                )
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        })
}
