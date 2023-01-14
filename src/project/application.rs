//! Application layer of the project entity.

use super::domain::Project;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ProjectRepository {
    async fn find_by_name(&self, name: &str) -> Result<Project>;
    async fn create(&self, project: &mut Project) -> Result<()>;
}

pub struct ProjectApplication<P: ProjectRepository> {
    pub project_repo: Arc<P>,
}
