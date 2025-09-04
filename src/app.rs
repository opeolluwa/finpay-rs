use std::sync::Arc;

use crate::users::users_router;
use axum::{Router, http::StatusCode, routing::get};
use sqlx::Pool;

pub fn load_routes(pool: Arc<Pool<sqlx::Postgres>>) -> Router {
    let router = Router::new();

    router
        .merge(users_router())
        .route("/health", get(async move || "Healthy..."))
    // .fallback(async || {
    //     ApiResponseBuilder::<()>::new()
    //         .message(
    //             "the resource you're looking does not exist or it has been permanently moved",
    //         )
    //         .status_code(StatusCode::NOT_FOUND)
    //         .build()
    //         .into_response()
    // })
}
