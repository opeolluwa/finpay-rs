use tower_http::cors::AllowOrigin;

use finpay_utils::extract_env;
use crate::config::kafka::KafkaProducer;
use crate::errors::AppError;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub max_db_connections: u32,
    pub body_limit_mb: usize,
    pub upload_path: String,
    pub export_path: String,
    pub port: u16,
    pub environment: String,
    pub allowed_origins: AllowOrigin,
    pub kafka_broker: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let port = extract_env::<u16>("PORT");

        let max_db_connections = extract_env::<u32>("MAX_DB_CONNECTIONS");

        let body_limit_mb = extract_env::<usize>("BODY_LIMIT_MB");

        let export_path = extract_env("EXPORT_PATH");
        let upload_path = extract_env("UPLOAD_PATH");

        let environment = extract_env("ENVIRONMENT");

        // Parse allowed origins (comma-separated list)
        let allowed_origins_str = extract_env::<String>("ALLOWED_ORIGINS");
        let allowed_origins = if allowed_origins_str == "*" {
            AllowOrigin::any()
        } else {
            let origins: Vec<_> = allowed_origins_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            AllowOrigin::list(origins.into_iter().map(|s| s.parse().unwrap()))
        };

        let kafka_broker = extract_env::<String>("KAFKA_BROKER");
        let database_url = extract_env("DATABASE_URL");
        Ok(Self {
            database_url,
            max_db_connections,
            body_limit_mb,
            upload_path,
            export_path,
            port,
            environment,
            allowed_origins,
            kafka_broker,
        })
    }
}
