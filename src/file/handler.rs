//! Event handler implementation for consuming file related events.

use crate::project::application::{ProjectApplication, ProjectRepository};

/// Handles file related events.
pub struct FileEventHandler<P: ProjectRepository> {
    project_app: ProjectApplication<P>,
    issuers_blacklist: Vec<String>,
}

impl<P: ProjectRepository> FileEventHandler<P> {}
