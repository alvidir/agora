//! Application layer of the project entity.

use super::domain::Project;
use crate::{
    metadata::domain::Metadata,
    result::{Error, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ProjectRepository {
    async fn find_by_name(&self, user_id: &str, name: &str) -> Result<Project>;
    async fn find_all(&self, user_ud: &str) -> Result<Vec<Project>>;
    async fn create(&self, project: &mut Project) -> Result<()>;
}

pub struct ProjectApplication<P: ProjectRepository> {
    pub project_repo: Arc<P>,
}

impl<P: ProjectRepository> ProjectApplication<P> {
    pub async fn create(&self, uid: &str, name: &str) -> Result<Project> {
        info!("processing a \"create\" project request for user {} ", uid);

        let Err(Error::NotFound) = self.project_repo.find_by_name(uid, name).await else {
            return Err(Error::AlreadyExists);
        };

        let meta = Metadata::new(uid);
        let mut project = Project::new("", name, meta);
        self.project_repo.create(&mut project).await?;

        Ok(project)
    }

    pub async fn list(&self, uid: &str) -> Result<Vec<Project>> {
        info!("processing a \"list\" projects request for user {} ", uid);
        Ok(self.project_repo.find_all(uid).await?)
    }
}
