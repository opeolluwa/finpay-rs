#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use crate::router::load_routes;
    use axum_test::TestServer;

    #[tokio::test]
    #[sqlx::test]
    async fn should_return_404(pool: sqlx::PgPool) {
        let app = load_routes(pool.into());

        let test_server = TestServer::new(app).expect("unable to create test server");

        let response = test_server.get("not_found").await;

        assert_eq!(response.status_code().clone(), StatusCode::NOT_FOUND);
    }

}