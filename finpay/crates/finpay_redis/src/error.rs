use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisClientError {
    #[error("an internal error occured due to redis client")]
    RedisError(#[from] RedisError),
}
