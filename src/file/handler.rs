//! Event handler implementation for consuming file related events.

use std::sync::Arc;

use crate::{
    project::application::{ProjectApplication, ProjectRepository},
    rabbitmq::EventHandler,
    result::{Error, Result},
};

use super::bus::FileEventPayload;

// #[async_trait::async_trait]
// pub trait ProjectApplication {
//     async fn create_with_id(&self, id: &str, uid: &str, name: &str) -> Result<Project>;
// }

pub struct FileEventHandler<'a, P: ProjectRepository> {
    pub issuers_whitelist: &'a [String],
    pub project_app: Arc<ProjectApplication<P>>,
}

#[async_trait::async_trait]
impl<'a, P: ProjectRepository + Sync + Send> EventHandler for FileEventHandler<'a, P> {
    async fn on_event(&self, body: Vec<u8>) -> Result<()> {
        let payload = bincode::deserialize::<FileEventPayload>(&body).map_err(|err| {
            warn!("{} deserializing file event body: {}", Error::Unknown, err);
            Error::Unknown
        })?;

        if !self.issuers_whitelist.contains(&payload.issuer.to_string()) {
            info!("discarting file event from issuer {}", payload.issuer);
            return Ok(());
        }

        match payload.kind {
            crate::rabbitmq::EventKind::Created => self.on_file_created(payload).await,
            _ => {
                warn!("unhandled file {} event", payload.kind);
                Ok(())
            }
        }
    }
}

impl<'a, P: ProjectRepository> FileEventHandler<'a, P> {
    async fn on_file_created(&self, event: FileEventPayload<'a>) -> Result<()> {
        info!(
            "handlering a file \"created\" event from issuer {}",
            event.issuer
        );

        self.project_app
            .create(event.file_id, event.user_id, event.file_name)
            .await
            .map(|_| ())
    }
}
