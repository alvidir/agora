//! Event handlers implementations

use crate::project::application::{ProjectApplication, ProjectRepository};

/// Handles file related events.
pub struct FileEventHandler<P: ProjectRepository> {
    project_app: ProjectApplication<P>,
    issuers: Vec<(String, bool)>,
}
