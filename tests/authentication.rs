use axum::http::StatusCode;
use axum_test::TestServer;
use fake::{Fake, Faker};
use lib_finpay_rs::router::load_routes;
use lib_finpay_rs::users::adapters::CreateUserRequest;
use lib_finpay_rs::users::enums::AccountType;
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_account(pool: PgPool) {
    let app = load_routes(pool.into());
    let test_server = TestServer::new(app).expect("failed to create test server");

    let test_user: CreateUserRequest = CreateUserRequest {
        first_name: "Alex".to_string(),
        last_name: "Murinto".to_string(),
        email: "alex.murinto@mailer.com".to_string(),
        password: Faker.fake(),
        account_type: AccountType::Freelancer,
        country: Faker.fake(),
        country_code: "NGN".to_string(),
        address: Faker.fake(),
        phone_number: Faker.fake(),
        occupation: None,
    };

    let response = test_server.post("/auth/register").json(&test_user).await;

    response.assert_status(StatusCode::CREATED);
}
