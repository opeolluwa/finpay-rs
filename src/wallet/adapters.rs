
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreateWalletRequest {
    pub name: String,
    pub currency_identifier : String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateWalletRequest {}



#[derive(Serialize, Deserialize, Validate)]
pub struct DeleteWalletRequest {}


