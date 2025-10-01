use std::sync::Arc;

use crate::banks::router::banks_routes;
use crate::countries::router::country_routes;
use crate::wallet::router::wallet_routes;
use crate::{
    authentication::router::authentication_routers,
    state::AppState,
    users::users_router,
    utils::{ApiResponseBuilder, EmptyResponseBody},
};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::Pool;

pub fn load_routes(pool: Arc<Pool<sqlx::Postgres>>) -> Router {
    let router = Router::new();

    let state = AppState::new(pool);

    router
        .nest("/users",users_router(&state))
        .nest("/auth", authentication_routers(&state))
        .nest("/countries", country_routes(&state))
        .nest("/wallet", wallet_routes(&state))
        .nest("/banks", banks_routes(&state))
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
