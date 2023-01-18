//! RabbitMQ utilities for managing events handlering and emitions.

use crate::result::{Error, Result};
use futures_util::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct RabbitMqEventBus {
    chann: Channel,
}

/// Represents all the possible kind of events that may be handled or emited.
#[derive(strum_macros::Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum EventKind {
    Created,
    Deleted,
}

impl RabbitMqEventBus {
    /// Given an exchange name and a queue name performs the binding between them two.
    /// Notice that this method will create the queue on RabbitMq if it does not exists, but the exchange must
    /// be already present.
    pub async fn queue_bind(&self, exchange: &str, queue: &str) -> Result<()> {
        let queue_options = QueueDeclareOptions {
            durable: true,
            auto_delete: false,
            exclusive: false,
            nowait: false,
            passive: false,
        };

        self.chann
            .queue_declare(&queue, queue_options, FieldTable::default())
            .await
            .map_err(|err| {
                warn!("declaring rabbitmq queue {}: {}", queue, err);
                Error::Unknown
            })?;

        self.chann
            .queue_bind(
                &queue,
                &exchange,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|err| {
                warn!(
                    "{} binding rabbitmq queue {} with exchange {}: {}",
                    Error::Unknown,
                    queue,
                    exchange,
                    err
                );
                Error::Unknown
            })?;

        Ok(())
    }

    /// Given a queue name and an event handler, listens on the queue with that name and forwards every event's
    /// data to the handler.
    pub async fn consume<T: DeserializeOwned, H: Fn(T)>(
        &self,
        queue: &str,
        handler: H,
    ) -> Result<()> {
        let mut consumer = self
            .chann
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|err| {
                warn!(
                    "{} registering a consumer for queue {}: {}",
                    Error::Unknown,
                    queue,
                    err
                );

                Error::Unknown
            })?;

        info!("waiting for events from queue {}", queue);

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.map_err(|err| {
                error!(
                    "{} consuming event from queue {}: {}",
                    Error::Unknown,
                    queue,
                    err
                );

                Error::Unknown
            })?;

            if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
                error!(
                    "{} performing a delivery ack on queue {}: {}",
                    Error::Unknown,
                    queue,
                    err
                );

                continue;
            }

            let data: T = bincode::deserialize(&delivery.data).map_err(|err| {
                error!("{} deserializing event data: {}", Error::Unknown, err);
                Error::Unknown
            })?;

            handler(data);
        }

        Ok(())
    }
}
