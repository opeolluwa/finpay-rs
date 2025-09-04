use crate::errors::RepositoryError;
use crate::users::adapters::CreateUserRequest;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;
use ulid::Ulid;

pub struct UsersRepository {
    pool: Arc<Pool<Postgres>>,
}

impl UsersRepository {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait UsersRepositoryExt {
    async fn create_account(&self, payload: &CreateUserRequest) -> Result<(), RepositoryError>;
}

impl UsersRepositoryExt for UsersRepository {
    async fn create_account(&self, payload: &CreateUserRequest) -> Result<(), RepositoryError> {
        let query = r#"
        INSERT INTO
    users (
        identifier,
        first_name,
        last_name,
        email,
        password,
        account_type,
        country,
        address,
        phone_number,
        country_code,
        occupation
    )
    VALUES($1, $2, $3, $4, $5, %6, $7, $8, $9, $10, $11)
        "#;
        sqlx::query(query)
            .bind(Ulid::new().to_string())
            .bind(&payload.first_name)
            .bind(&payload.last_name)
            .bind(&payload.email)
            .bind(&payload.password)
            .bind(&payload.account_type.to_string())
            .bind(&payload.country)
            .bind(&payload.address)
            .bind(&payload.phone_number)
            .bind(&payload.country_code)
            .bind(&payload.occupation)
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }
}
