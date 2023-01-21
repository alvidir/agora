//! Application layer of the project entity.

use super::domain::Project;
use crate::{metadata::domain::Metadata, result::Result};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait ProjectRepository {
    async fn find_by_name(&self, user_id: &str, name: &str) -> Result<Project>;
    async fn find_all(&self, user_ud: &str) -> Result<Vec<Project>>;
    async fn create(&self, project: &mut Project) -> Result<()>;
}

#[async_trait::async_trait]
pub trait EventBus {
    async fn emit_project_created(&self, project: &Project) -> Result<()>;
}

pub struct ProjectApplication<P: ProjectRepository> {
    pub project_repo: Arc<P>,
}

impl<P: ProjectRepository> ProjectApplication<P> {
    pub async fn create(&self, id: &str, uid: &str, name: &str) -> Result<Project> {
        if id.is_empty() {
            info!("processing a \"create\" project request for user {}", uid);
        } else {
            info!(
                "processing a \"create\" with id {} project request for user {}",
                id, uid
            );
        }

        let meta = Metadata::new(uid);
        let mut project = Project::new(id, name, meta);
        self.project_repo.create(&mut project).await?;

        Ok(project)
    }

    pub async fn list(&self, uid: &str) -> Result<Vec<Project>> {
        info!("processing a \"list\" projects request for user {} ", uid);
        Ok(self.project_repo.find_all(uid).await?)
    }
}
