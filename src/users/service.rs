use crate::errors::ServiceError;
use crate::users::adapters::CreateUserRequest;
use crate::users::adapters::LoginUserRequest;
use crate::users::repositories::UsersRepository;

pub struct UserService {
    repository: UsersRepository,
}

impl UserService {
    pub fn new(repository: UsersRepository) -> Self {
        Self { repository }
    }
}

pub trait UsersServiceExt {
    async fn register(&self, payload: &CreateUserRequest) -> Result<(), ServiceError>;
    async fn login(&self, payload: &LoginUserRequest) -> Result<(), ServiceError>;
    async fn logout(&self) -> Result<(), ServiceError>;
    async fn forgot_password(&self) -> Result<(), ServiceError>;
    async fn reset_password(&self) -> Result<(), ServiceError>;
}

impl UsersServiceExt for UserService {
    async fn register(&self, payload: &CreateUserRequest) -> Result<(), ServiceError> {
        todo!()
    }

    async fn login(&self, payload: &LoginUserRequest) -> Result<(), ServiceError> {
        todo!()
    }

    async fn logout(&self) -> Result<(), ServiceError> {
        todo!()
    }

    async fn forgot_password(&self) -> Result<(), ServiceError> {
        todo!()
    }

    async fn reset_password(&self) -> Result<(), ServiceError> {
        todo!()
    }
}
