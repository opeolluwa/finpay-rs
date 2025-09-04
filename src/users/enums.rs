use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename_all = "lowercase", type_name="account_type_enum")]
pub enum AccountType {
    Freelancer,
    Company,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Freelancer => write!(f, "freelancer"),
            AccountType::Company => write!(f, "company"),
        }
    }
}
