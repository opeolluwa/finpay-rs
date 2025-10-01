use crate::errors::ServiceError;
use crate::security::otp::adapter::GenerateOtpRequest;
use crate::security::otp::repository::{OtpRepository, OtpRepositoryExt};
use chrono::{Local, TimeDelta};
use finpay_utils::generate_otp;
use uuid::Uuid;

const OTP_VALIDITY: TimeDelta = TimeDelta::minutes(10);

#[derive(Debug, Clone)]
pub struct OtpService {
    repository: OtpRepository,
}

impl OtpService {
    pub fn new(pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        Self {
            repository: OtpRepository::init(pool),
        }
    }
}

pub trait OtpServiceExt {
    fn new_otp_for_user(
        &self,
        user_identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<String, ServiceError>> + Send;

    fn validate_otp_for_user(
        &self,
        user_identifier: &Uuid,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<bool, ServiceError>> + Send;
}

impl OtpServiceExt for OtpService {
    async fn new_otp_for_user(&self, user_identifier: &Uuid) -> Result<String, ServiceError> {
        let token = generate_otp();
        let request = GenerateOtpRequest {
            token: token.to_owned(),
            user_identifier: user_identifier.to_owned(),
        };

        self.repository
            .new_with_user(&request)
            .await
            .map_err(ServiceError::from)?;

        Ok(token)
    }

    async fn validate_otp_for_user(
        &self,
        user_identifier: &Uuid,
        otp: &str,
    ) -> Result<bool, ServiceError> {
        if let Some(stored_otp) = self
            .repository
            .find_latest_by_user(user_identifier)
            .await
            .map_err(ServiceError::from)?
        {
            let now = Local::now();

            let is_not_expired = now - stored_otp.created_date <= OTP_VALIDITY;
            let is_match = stored_otp.token == otp;
            Ok(is_match && is_not_expired)
        } else {
            Ok(false)
        }
    }
}
