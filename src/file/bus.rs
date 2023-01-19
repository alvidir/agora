//! Event bus implementation for emiting file related events.

use std::borrow::Cow;
use std::sync::Arc;

use crate::rabbitmq::EventKind;
use crate::{
    project::{application::EventBus as ProjectEventBus, domain::Project},
    result::{Error, Result},
};
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel};
use serde::{Deserialize, Serialize};

/// Determines the data to be provided/expected when emiting/handlering a file related event.
#[derive(Serialize, Deserialize)]
pub struct FileEventPayload<'a> {
    pub(super) issuer: &'a str,
    pub(super) user_id: i32,
    pub(super) app_id: &'a str,
    pub(super) file_name: &'a str,
    pub(super) file_id: &'a str,
    pub(super) kind: EventKind,
}

pub struct RabbitMqFileBus<'a> {
    pub channel: Arc<&'a Channel>,
    pub issuer: &'a str,
    pub exchange: &'a str,
}

#[async_trait::async_trait]
impl<'a> ProjectEventBus for RabbitMqFileBus<'a> {
    async fn emit_project_created(&self, project: &Project) -> Result<()> {
        let event = FileEventPayload {
            issuer: "",
            user_id: 12,
            app_id: "",
            file_name: "",
            file_id: "",
            kind: EventKind::Created,
        };

        let payload = serde_json::to_string(&event)
            .map(|str| str.into_bytes())
            .map_err(|err| {
                error!(
                    "{} serializing \"project created\" event data to json: {}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        self.channel
            .basic_publish(
                self.exchange,
                "",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|err| {
                error!(
                    "{} emititng \"project created\" event: {}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?
            .await
            .map_err(|err| {
                error!(
                    "{} confirming \"project created\" event reception: {}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        Ok(())
    }
}
