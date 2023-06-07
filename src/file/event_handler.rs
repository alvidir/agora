//! Event handler implementation for consuming file related events.

use super::event_bus::FileEventPayload;
use crate::{
    project::application::{CreateOptions, EventBus, ProjectApplication, ProjectRepository},
    rabbitmq::EventHandler,
    result::{Error, Result},
};

pub struct FileEventHandler<P: ProjectRepository, B: EventBus> {
    pub issuers_whitelist: &'static [String],
    pub project_app: ProjectApplication<P, B>,
}

#[async_trait::async_trait]
impl<P: ProjectRepository + Sync + Send, B: EventBus + Sync + Send> EventHandler
    for FileEventHandler<P, B>
{
    async fn on_event(&self, body: Vec<u8>) -> Result<()> {
        let payload = bincode::deserialize::<FileEventPayload>(&body).map_err(|err| {
            warn!("{} deserializing file event body: {}", Error::Unknown, err);
            Error::Unknown
        })?;

        if !self
            .issuers_whitelist
            .contains(&payload.event_issuer.to_string())
        {
            info!("discarting file event from issuer {}", payload.event_issuer);
            return Ok(());
        }

        match payload.event_kind {
            crate::rabbitmq::EventKind::Created => self.on_file_created(payload).await,
            _ => {
                warn!("unhandled file {} event", payload.event_kind);
                Ok(())
            }
        }
    }
}

impl<P: ProjectRepository, B: EventBus> FileEventHandler<P, B> {
    async fn on_file_created<'a>(&self, event: FileEventPayload<'a>) -> Result<()> {
        info!(
            "handlering a file \"created\" event from issuer {}",
            event.event_issuer
        );

        self.project_app
            .create(
                event.file_name,
                event.user_id,
                CreateOptions {
                    reference: Some(event.file_id.to_string()),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }
}
