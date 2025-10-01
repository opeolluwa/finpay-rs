use std::path::Path;

use sqlx::{Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};

use crate::{config::AppConfig, errors::AppError};

pub struct AppDatabase {}

impl AppDatabase {
    pub async fn init(config: &AppConfig) -> Result<Pool<Postgres>, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;
        tracing::info!("Database initialized");

        let migrator = Migrator::new(Path::new("migrations"))
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;
        migrator
            .run(&pool)
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        tracing::info!("Database migrations completed");
        Ok(pool)
    }
}
