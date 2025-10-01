use finpay_mailer::EmailClient;

pub struct AppBackgroundTasks {}

impl AppBackgroundTasks {
    pub fn run() {
        tokio::task::spawn(async move {
            match EmailClient::new().test_connection() {
                Ok(true) => tracing::info!("SMTP Connection established"),
                Ok(false) => tracing::warn!("Connection test failed"),
                Err(e) => tracing::error!("Error testing connection: {}", e),
            };
        });
    }
}
