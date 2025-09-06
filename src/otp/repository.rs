use crate::errors::RepositoryError;
use crate::otp::adapter::GenerateOtpRequest;
use crate::otp::entities::Otp;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OtpRepository {
    pool: Pool<Postgres>,
}

impl OtpRepository {
    pub fn init(pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool: pool.clone() }
    }
}

pub trait OtpRepositoryExt {
    fn new_with_user(
        &self,
        generate_otp_request: &GenerateOtpRequest,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;

    fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Otp>, RepositoryError>> + Send;

    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<Option<Otp>, RepositoryError>> + Send;

    fn delete_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}

impl OtpRepositoryExt for OtpRepository {
    async fn new_with_user(
        &self,
        generate_otp_request: &GenerateOtpRequest,
    ) -> Result<(), RepositoryError> {
        let otp_identifier = Uuid::new_v4();
        let user_identifier = generate_otp_request.user_identifier;

        sqlx::query(r#"INSERT INTO one_time_passwords (identifier, user_identifier, token) VALUES ($1,$2,$3)"#)
            .bind(otp_identifier).bind(user_identifier)
            .bind(&generate_otp_request.token)
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::SqlxError)?;

        Ok(())
    }

    async fn find_latest_by_user(
        &self,
        user_identifier: &Uuid,
    ) -> Result<Option<Otp>, RepositoryError> {
        sqlx::query_as::<_, Otp>(
            r#"SELECT * FROM one_time_passwords WHERE user_identifier = $1 ORDER BY created_at DESC LIMIT 1"#,
        ).bind(user_identifier)
            .fetch_optional(&self.pool)
            .await.map_err(RepositoryError::from)
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Result<Option<Otp>, RepositoryError> {
        sqlx::query_as::<_, Otp>(r#"SELECT * FROM one_time_passwords WHERE identifier = $1"#)
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::from)
    }

    async fn delete_by_identifier(&self, identifier: &Uuid) -> Result<(), RepositoryError> {
        sqlx::query(r#"DELETE FROM one_time_passwords WHERE identifier = $1"#)
            .bind(identifier)
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

        Ok(())
    }
}
