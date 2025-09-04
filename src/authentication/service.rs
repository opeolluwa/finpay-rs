use crate::{errors::ServiceError, users::{adapters::{CreateUserRequest, LoginUserRequest}, service::UsersService}};

pub struct AuthenticationService {
    user_service: UsersService,
}

impl AuthenticationService {
    pub fn new(user_service: UsersService) -> Self {
        Self { user_service }
    }
}

pub trait AuthenticationServiceExt {
    async fn register(&self, payload: &CreateUserRequest) -> Result<(), ServiceError>;
    async fn login(&self, payload: &LoginUserRequest) -> Result<(), ServiceError>;
    async fn logout(&self) -> Result<(), ServiceError>;
    async fn forgot_password(&self) -> Result<(), ServiceError>;
    async fn reset_password(&self) -> Result<(), ServiceError>;
}


impl AuthenticationServiceExt for AuthenticationService  {
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