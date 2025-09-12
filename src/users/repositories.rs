use crate::errors::RepositoryError;
use crate::shared::repository::DatabaseInsertResult;
use crate::users::adapters::CreateUserRequest;
use crate::users::entities::User;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UsersRepository {
    pool: Pool<Postgres>,
}

impl UsersRepository {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait UsersRepositoryExt {
    fn create_account(
        &self,
        payload: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<DatabaseInsertResult, RepositoryError>> + Send;

    fn find_user_by_pk(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn find_user_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn set_verified(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn set_new_password(
        &self,
        user_identifier: &Uuid,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}

impl UsersRepositoryExt for UsersRepository {
    async fn create_account(
        &self,
        payload: &CreateUserRequest,
    ) -> Result<DatabaseInsertResult, RepositoryError> {
        let identifier = Uuid::new_v4();

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
    VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING identifier
        "#;
        sqlx::query_as::<_, DatabaseInsertResult>(query)
            .bind(identifier)
            .bind(&payload.first_name)
            .bind(&payload.last_name)
            .bind(&payload.email)
            .bind(&payload.password)
            .bind(payload.account_type)
            .bind(&payload.country)
            .bind(&payload.address)
            .bind(&payload.phone_number)
            .bind(&payload.country_code)
            .bind(&payload.occupation)
            .fetch_one(&self.pool)
            .await
            .map_err(RepositoryError::from)
    }

    async fn find_user_by_pk(&self, identifier: &Uuid) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE identifier = $1")
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::from)
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::from)
    }

    async fn set_verified(&self, user_identifier: &Uuid) -> Result<(), RepositoryError> {
        sqlx::query(
            "UPDATE users SET is_verified = $1,  updated_at = NOW () WHERE identifier = $2",
        )
        .bind(true)
        .bind(user_identifier)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn set_new_password(
        &self,
        user_identifier: &Uuid,
        new_password: &str,
    ) -> Result<(), RepositoryError> {
        let query = "UPDATE users SET password = $1 WHERE identifier = $2";
        sqlx::query(query)
            .bind(new_password)
            .bind(user_identifier)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), RepositoryError> {
        sqlx::query(r#"UPDATE users SET profile_picture = $2 WHERE identifier = $1"#)
            .bind(user_identifier)
            .bind(avatar_url)
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let repository = UsersRepository::new(&pool);
        let create_user_request: CreateUserRequest = Faker.fake();

        // Create a new user
        let new_user = repository
            .create_account(&create_user_request)
            .await
            .expect("failed to create user");

        let new_user_id = new_user.identifier;
        assert!(!Uuid::is_nil(&new_user_id));

        // Fetch the user back
        let fetched = repository
            .find_user_by_pk(&new_user_id)
            .await
            .expect("failed to fetch created user")
            .unwrap();

        // Verify persistence
        assert_eq!(fetched.identifier, new_user_id);
        assert_eq!(fetched.email, create_user_request.email);

        // Optional: verify defaults/constraints
        assert_eq!(fetched.is_verified, false); // if default status is "active"
        assert!(fetched.created_date <= chrono::Utc::now());
    }
}
