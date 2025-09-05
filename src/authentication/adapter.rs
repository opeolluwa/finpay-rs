use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::users::{adapters::CreateUserRequest, enums::AccountType};

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
