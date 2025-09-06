use crate::authentication::claims::Claims;
use crate::users::{adapters::CreateUserRequest, enums::AccountType};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    #[validate(length(min = 1, message = "first name is required", code = "first name"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last name is required", code = "last name"))]
    pub last_name: String,
    #[validate(email(message = "please provide a valid email"))]
    pub email: String,
    #[validate(length(
        min = 8,
        message = "password cannot be less than 8 characters",
        code = "password"
    ))]
    pub password: String,
    #[validate(custom(
        function = "validate_account_type",
        code = "account type",
        message = "account type can only be one of \"freelancer\" or \"company\"",
    ))]
    pub account_type: String,
    #[validate(length(min = 1, code = "country", message = "please choose a valid country"))]
    pub country: String,
    #[validate(length(
        min = 1,
        max = 5,
        message = "invalid country code",
        code = "country code"
    ))]
    pub country_code: String,
    #[validate(length(min = 1, message = "address is required", code = "address"))]
    pub address: String,
    #[validate(length(min = 1, message = "invalid phone number", code = "phone number"))]
    pub phone_number: String,
    #[validate(length(
        min = 1,
        message = "please provide valid occupation",
        code = "occupation"
    ))]
    pub occupation: Option<String>,
}

impl Into<CreateUserRequest> for CreateAccountRequest {
    fn into(self) -> CreateUserRequest {
        CreateUserRequest {
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: self.password,
            account_type: self.account_type.into(),
            country: self.country,
            country_code: self.country_code,
            address: self.address,
            phone_number: self.phone_number,
            occupation: self.occupation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[validate(email(message = "email is required"))]
    pub email: String,
    #[validate(length(min = 1, message = "password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ForgottenPasswordRequest {
    #[validate(email(message = "email is required"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNewPasswordRequest {
    #[validate(length(min = 1, message = "password cannot be empty"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "password does  not match"))]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAccountRequest {
    #[validate(length(message = "otp is required", code = "otp", max = 6))]
    pub otp: String,
}

pub type RefreshTokenRequest = Claims;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub iat: i64,
    pub exp: i64,
    pub refresh_token_exp: i64,
    pub refresh_token_iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgottenPasswordResponse {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNewPasswordResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAccountResponse {}

pub type RefreshTokenResponse = LoginResponse;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

fn validate_account_type(account_type: &str) -> Result<(), ValidationError> {
    if account_type != AccountType::Company.to_string()
        && account_type != AccountType::Freelancer.to_string()
    {
        return Err(ValidationError::new(
            "account type can only be one of \"freelancer\" or \"company\"",
        ));
    }

    Ok(())
}
