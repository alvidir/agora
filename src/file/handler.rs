//! Event handler implementation for consuming file related events.

use crate::{
    project::domain::Project,
    rabbitmq::EventHandler,
    result::{Error, Result},
};

use super::bus::FileEventPayload;

#[async_trait::async_trait]
pub trait ProjectApplication {
    async fn create_with_id(&self, id: &str, uid: &str, name: &str) -> Result<Project>;
}

pub struct FileEventHandler<'a, P: ProjectApplication + Sync> {
    pub issuers_whitelist: &'a [String],
    pub project_app: P,
}

#[async_trait::async_trait]
impl<'a, P: ProjectApplication + Sync> EventHandler for FileEventHandler<'a, P> {
    async fn on_event(&self, body: Vec<u8>) {
        let Ok(payload) = bincode::deserialize::<FileEventPayload>(&body).map_err(|err| {
            warn!("{} deserializing file event body: {}", Error::Unknown, err);
            Error::Unknown
        }) else {
            return;
        };

        if !self.issuers_whitelist.contains(&payload.issuer.to_string()) {
            info!("discarting event from issuer {}", payload.issuer);
            return;
        }

        match payload.kind {
            crate::rabbitmq::EventKind::Created => self.on_file_created(payload).await,
            _ => {
                warn!("unhandled file event with kind {}", payload.kind)
            }
        }
    }
}

impl<'a, P: ProjectApplication + Sync> FileEventHandler<'a, P> {
    async fn on_file_created(&self, event: FileEventPayload<'a>) {}
}
