#![warn(unused_extern_crates)]

use lib_midgard::app::create_cors_layer;
use lib_midgard::app::shutdown_signal;
use lib_midgard::config::AppConfig;
use lib_midgard::database::AppDatabase;
use lib_midgard::errors::AppError;
use lib_midgard::logger::AppLogger;
use lib_midgard::router::load_routes;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
use tower_http::timeout::TimeoutLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    AppLogger::init();
    tracing::info!("Logger initialized");

    let config = AppConfig::from_env()?;

    tracing::info!("App Config loaded!");

    let db_pool = AppDatabase::init(&config).await?;
    let shared_db_pool = std::sync::Arc::new(db_pool);

    let app = load_routes(shared_db_pool)
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
