Use sqlx::test for repository unit tests (each test gets its own DB).

Use service trait + mock repos for pure business logic tests.

Use real DB + Axum router + tower::ServiceExt for integration tests.

GitHub Actions spins up Postgres and runs migrations.