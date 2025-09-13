use axum::http::StatusCode;

use axum_test::TestServer;
use lib_finpay_rs::router::load_routes;

#[sqlx::test]
async fn should_return_404(pool: sqlx::PgPool) {
    let app = load_routes(pool.into());

    let test_server = TestServer::new(app).expect("unable to create test server");

    let response = test_server.get("not_found").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn should_return_healthy(pool: sqlx::PgPool) {
    let app = load_routes(pool.into());

    let test_server = TestServer::new(app).expect("unable to create test server");

    let response = test_server.get("/health").await;

    response.assert_text("Healthy...");
    response.assert_status(StatusCode::OK);
}
