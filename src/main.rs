#![warn(unused_extern_crates)]

use axum::extract::DefaultBodyLimit;
use lib_finpay_rs::{
    config::{
        AppConfig,
        app::{create_cors_layer, shutdown_signal},
        database::AppDatabase,
        filesystem::AppFileSystem,
        logger::AppLogger,
        tasks::AppBackgroundTasks,
    },
    errors::AppError,
    router::load_routes,
};

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    AppLogger::init();
    tracing::info!("Logger initialized");

    let config = AppConfig::from_env()?;
    AppFileSystem::init(&config)?;
    tracing::info!("App Config loaded!");

    let db_pool = AppDatabase::init(&config).await?;
    let shared_db_pool = std::sync::Arc::new(db_pool);

    AppBackgroundTasks::run();
    tracing::info!("Background tasks initialized");

    let body_limit_bytes = config.body_limit_mb * 1024 * 1024;

    let app = load_routes(shared_db_pool)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(body_limit_bytes))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(create_cors_layer(&config));

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));
    tracing::info!("Application listening on http://{ip_address}");

    let listener = tokio::net::TcpListener::bind(ip_address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
