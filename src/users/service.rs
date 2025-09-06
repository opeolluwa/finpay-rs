use std::sync::Arc;

use bcrypt::DEFAULT_COST;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::errors::RepositoryError;
use crate::errors::RepositoryError::RecordNotFound;
use crate::errors::ServiceError;
use crate::users::adapters::CreateUserRequest;
use crate::users::adapters::LoginUserRequest;
use crate::users::entities::User;
use crate::users::repositories::UsersRepository;
use crate::users::repositories::UsersRepositoryExt;

#[derive(Debug, Clone)]
pub struct UsersService {
    repository: UsersRepository,
}

impl UsersService {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self {
            repository: UsersRepository::new(&pool),
        }
    }
    fn hash_password(&self, password: &str) -> Result<String, ServiceError> {
        let hash = bcrypt::hash(password, DEFAULT_COST)?;

        Ok(hash)
    }
}

pub trait UsersServiceExt {
    fn create_account(
        &self,
        payload: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<Uuid, ServiceError>> + Send;

    fn find_user_by_pk(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<User, ServiceError>> + Send;

    fn find_user_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<User, ServiceError>> + Send;
}

impl UsersServiceExt for UsersService {
    async fn create_account(&self, payload: &CreateUserRequest) -> Result<Uuid, ServiceError> {
        let hashed_password = self.hash_password(&payload.password)?;

        let request = CreateUserRequest {
            password: hashed_password,
            ..payload.clone()
        };

        let result = self.repository.create_account(&request).await?;
        Ok(result.identifier)
    }

    async fn find_user_by_pk(&self, identifier: &Uuid) -> Result<User, ServiceError> {
        self.repository
            .find_user_by_pk(identifier)
            .await?
            .ok_or(ServiceError::RepositoryError(RecordNotFound))
    }

    async fn find_user_by_email(&self, email: &str) -> Result<User, ServiceError> {
        self.repository
            .find_user_by_email(email)
            .await?
            .ok_or(ServiceError::RepositoryError(RecordNotFound))
    }
}
