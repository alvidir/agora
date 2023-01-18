//! Event bus implementation for emiting file related events.

use crate::rabbitmq::{EventKind, RabbitMqEventBus};
use crate::{
    project::{application::EventBus as ProjectEventBus, domain::Project},
    result::{Error, Result},
};
use lapin::Channel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

pub struct RabbitMqFileBus {
    bus: RabbitMqEventBus,
}

#[async_trait::async_trait]
impl ProjectEventBus for RabbitMqFileBus {
    async fn emit_project_created(&self, project: &Project) -> Result<()> {
        todo!()
    }
}
