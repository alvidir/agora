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
    async fn find_by_created_by_and_name(&self, user_id: &str, name: &str) -> Result<Project>;
    async fn create(&self, project: &mut Project) -> Result<()>;
}

pub struct ProjectApplication<P: ProjectRepository> {
    pub project_repo: Arc<P>,
}

impl<P: ProjectRepository> ProjectApplication<P> {
    pub async fn create(&self, uid: &str, name: &str) -> Result<Project> {
        info!("processing a \"create\" project request for user {} ", uid);

        let Err(Error::NotFound) = self.project_repo.find_by_created_by_and_name(uid, name).await else {
            return Err(Error::AlreadyExists);
        };

        let meta = Metadata::new(uid);
        let mut project = Project::new("", name, meta);
        self.project_repo.create(&mut project).await?;

        Ok(project)
    }
}
