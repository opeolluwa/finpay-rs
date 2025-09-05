use crate::{
    errors::ServiceError,
    users::{
        adapters::{CreateUserRequest, LoginUserRequest},
        entities::User,
        service::{UsersService, UsersServiceExt},
    },
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_service: UsersService,
}

impl AuthenticationService {
    pub fn init(user_service: UsersService) -> Self {
        Self { user_service }
    }
}

pub trait AuthenticationServiceExt {
    fn register(
        &self,
        payload: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<User, ServiceError>> + Send;
    fn login(
        &self,
        payload: &LoginUserRequest,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn logout(&self) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn forgot_password(&self)
    -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
    fn reset_password(&self) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AuthenticationServiceExt for AuthenticationService {
    async fn register(&self, payload: &CreateUserRequest) -> Result<User, ServiceError> {
        let user_iddentifier = self.user_service.create_account(&payload).await?;

        let user = self.user_service.find_user_by_pk(&user_iddentifier).await?;

        Ok(user)
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
