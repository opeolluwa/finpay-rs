use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletRequest {
    #[validate(length(min = 1, message = "name is required", code = "wallet name"))]
    pub name: String,
    #[validate(length(
        equal = 36,
        message = "invalid currency selected",
        code = "wallet::currency::"
    ))]
    pub currency_identifier: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateWalletRequest {}

#[derive(Serialize, Deserialize, Validate)]
pub struct DeleteWalletRequest {}
