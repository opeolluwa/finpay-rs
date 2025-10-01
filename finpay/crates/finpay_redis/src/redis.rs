use finpay_utils::extract_env;
use redis::{
    AsyncCommands,
    aio::{ConnectionManager, ConnectionManagerConfig},
};

use crate::RedisClientError;

pub struct RedisClient {
    connection_manager: ConnectionManager,
}

impl RedisClient {
    pub async fn new() -> Result<Self, RedisClientError> {
        let redis_connection_url: String = extract_env("REDIS_CONNECTION_URL");
        let redis_client =
            redis::Client::open(redis_connection_url).map_err(RedisClientError::RedisError)?;

        let config = ConnectionManagerConfig::new().set_number_of_retries(5);
        let connection_manager =
            redis::aio::ConnectionManager::new_with_config(redis_client, config)
                .await
                .map_err(|err| {
                    log::error!("failed to create redis connection manager due to {err}");
                    RedisClientError::RedisError(err)
                })?;

        Ok(Self { connection_manager })
    }

    pub fn get_connection(&mut self) -> ConnectionManager {
        self.connection_manager.clone()
    }
}

pub trait RedisClientExt {
    fn blacklist_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<(), RedisClientError>> + Send;
    fn save_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<(), RedisClientError>> + Send;
    fn fetch_refresh_token(
        &mut self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<Option<String>, RedisClientError>> + Send;

    fn get_token_ttl(
        &mut self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<u64, RedisClientError>>;

    fn check_blacklisted_token(
        &mut self,
        token: &str,
    ) -> impl Future<Output = Result<Option<String>, RedisClientError>> + Send;
}

impl RedisClientExt for RedisClient {
    async fn blacklist_refresh_token(&mut self, token: &str) -> Result<(), RedisClientError> {
        let key = &format!("blacklist_token:{token}");
        let stored_token = self.fetch_refresh_token(token).await?;
        if stored_token.is_some() {
            let key = format!("refresh_token:{}", stored_token.unwrap());
            let ttl = self.get_token_ttl(&key).await?;
            let () = self
                .connection_manager
                .set_ex(key, token, ttl)
                .await
                .map_err(RedisClientError::from)?;
        }
        let refresh_token_validity_in_minutes: u64 = extract_env("REFRESH_TOKEN_TTL_IN_MINUTES");
        let validity_secs = refresh_token_validity_in_minutes * 60;

        let _: () = self
            .connection_manager
            .set_ex(key, token, validity_secs)
            .await
            .map_err(RedisClientError::from)?;

        Ok(())
    }

    async fn save_refresh_token(&mut self, token: &str) -> Result<(), RedisClientError> {
        let key = format!("refresh_token:{token}");
        let refresh_token_validity_in_minutes: u64 = extract_env("REFRESH_TOKEN_TTL_IN_MINUTES");
        let validity_secs = refresh_token_validity_in_minutes * 60;

        let _: () = self
            .connection_manager
            .set_ex(key, token, validity_secs)
            .await
            .map_err(RedisClientError::from)?;

        Ok(())
    }

    async fn fetch_refresh_token(
        &mut self,
        token: &str,
    ) -> Result<Option<String>, RedisClientError> {
        let key = &format!("refresh_token:{token}");
        let result: Option<String> = self
            .connection_manager
            .get(key)
            .await
            .map_err(RedisClientError::from)?;

        Ok(result)
    }

    async fn get_token_ttl(&mut self, key: &str) -> Result<u64, RedisClientError> {
        let result: u64 = self
            .connection_manager
            .ttl(key)
            .await
            .map_err(RedisClientError::from)?;

        Ok(result)
    }

    async fn check_blacklisted_token(
        &mut self,
        token: &str,
    ) -> Result<Option<String>, RedisClientError> {
        let key = &format!("blacklisted_token:{token}");
        let result: Option<String> = self
            .connection_manager
            .get(key)
            .await
            .map_err(RedisClientError::from)?;

        Ok(result)
    }
}
