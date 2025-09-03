use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub profile_picture: Option<String>
}