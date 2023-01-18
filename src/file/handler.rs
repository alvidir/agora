//! Event handler implementation for consuming file related events.

use crate::{
    project::application::{ProjectApplication, ProjectRepository},
    rabbitmq::EventHandler,
    result::Result,
};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait OnFileEvent {}

/// Handles file related events.
pub struct FileEventHandler<'a, P: ProjectRepository> {
    pub project_app: Arc<&'a ProjectApplication<P>>,
    pub issuers_blacklist: &'a Vec<&'a str>,
}

// #[async_trait::async_trait]
// impl<'a, P: ProjectRepository> EventHandler for FileEventHandler<'a, P> {
//     async fn on_event(&self, body: Vec<u8>) -> Result<()> {
//         Ok(())
//     }
// }
