use crate::config::AppConfig;
use crate::errors::AppError;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use futures::StreamExt;

pub struct KafkaProducer {
    stream: FutureProducer,
}

#[derive(Serialize, Debug)]
pub struct KafkaMessage<T>
where
    T: Serialize + DeserializeOwned + std::fmt::Debug + std::fmt::Display,
{
    pub topic: String,
    pub message: T,
}

impl KafkaProducer {
    pub async fn new() -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &app_config.kafka_broker)
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
        T: Serialize + DeserializeOwned + std::fmt::Debug + std::fmt::Display,
    {
        let serialized_message = serde_json::to_string(&message).map_err(|err| {
            tracing::error!("Failed to serialize kafka message: {err}");
            AppError::OperationFailed("failed to serialize kafka message".to_string())
        })?;

        let record = FutureRecord::to(&message.topic)
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

pub struct KafkaConsumer {
    stream: StreamConsumer,
}

impl KafkaConsumer {
    pub async fn new(group_id: &str, topics: &[&str]) -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &app_config.kafka_broker)
            .set("group.id", group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .map_err(|err| {
                tracing::error!("Failed to create kafka consumer: {err}");
                AppError::OperationFailed("failed to create kafka consumer".to_string())
            })?;

        consumer.subscribe(topics).map_err(|err| {
            tracing::error!("Failed to subscribe to topics: {err}");
            AppError::OperationFailed("failed to subscribe to kafka topics".to_string())
        })?;

        Ok(Self { stream: consumer })
    }

}
