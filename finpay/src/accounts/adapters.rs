use uuid::Uuid;

pub struct AccountCreationParams {
    pub user_identifier: Uuid,
    pub country_identifier: Uuid,
}
