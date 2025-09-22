use crate::config::AppConfig;
use crate::errors::AppError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

pub struct KafkaProducer {
    stream: FutureProducer,
}

#[derive(Serialize, Debug)]
pub struct KafkaMessage<T>
where
    T: serde::Serialize + DeserializeOwned + std::fmt::Debug + std::fmt::Display,
{
    pub topic: String,
    pub message: T,
}

impl KafkaProducer {
    pub async fn new() -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;

        let producer: FutureProducer = ClientConfig::default()
            .set("bootstrap.servers", app_config.kafka_broker)
            .set("message.timeout.ms", "5000")
            .create()
            .map_err(|err| {
                tracing::error!("Failed to create kafka client: {err}");
                AppError::OperationFailed("failed to create kafka client".to_string())
            })?;

        Ok(Self { stream: producer })
    }

    pub async fn send<T>(&self, message: KafkaMessage<T>) -> Result<(), AppError>
    where
        T: serde::Serialize + DeserializeOwned + std::fmt::Debug + std::fmt::Display,
    {
        let serialized_message = serde_json::to_string(&message).map_err(|err| {
            tracing::error!("Failed to serialize kafka message: {err}");
            AppError::OperationFailed("failed to serialize kafka message".to_string())
        })?;

        let record = FutureRecord::to(message.topic.as_str())
            .payload(&serialized_message)
            .key("test");

        let delivery_status = self
            .stream
            .send(record, Timeout::After(Duration::from_secs(20)))
            .await;

        match delivery_status {
            Ok(_) => Ok(()),
            Err((kafka_error, owned_message)) => Err(AppError::OperationFailed(format!(
                "failed to send {owned_message:?} due to {kafka_error}"
            ))),
        }
    }
}
