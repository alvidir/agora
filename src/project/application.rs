//! Application layer of the project entity.

use super::domain::Project;
use crate::{metadata::domain::Metadata, result::Result};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait ProjectRepository {
    async fn find(&self, id: &str, created_by: &str) -> Result<Project>;
    async fn find_all(&self, created_by: &str) -> Result<Vec<Project>>;
    async fn create(&self, project: &mut Project) -> Result<()>;
    async fn update(&self, project: &Project) -> Result<()>;
}
#[async_trait::async_trait]
pub trait EventBus {
    async fn emit_file_created(&self, project: &Project) -> Result<()>;
}

#[derive(Default, Clone)]
pub struct CreateOptions {
    pub description: Option<String>,
    pub reference: Option<String>,
}

pub struct ProjectApplication<P: ProjectRepository, B: EventBus> {
    pub project_repo: Arc<P>,
    pub event_bus: Arc<B>,
}

impl<P: ProjectRepository, B: EventBus> ProjectApplication<P, B> {
    pub async fn get(&self, id: &str, created_by: &str) -> Result<Project> {
        info!("processing a \"get\" project request for user {created_by} ",);
        self.project_repo.find(id, created_by).await
    }

    pub async fn list(&self, uid: &str) -> Result<Vec<Project>> {
        info!("processing a \"list\" projects request for user {} ", uid);
        self.project_repo.find_all(uid).await
    }

    pub async fn create(
        &self,
        name: &str,
        created_by: &str,
        options: CreateOptions,
    ) -> Result<Project> {
        info!("processing a \"create\" project request for user {created_by}",);

        let meta = Metadata::new(created_by);
        let mut project = Project {
            id: "".to_string(),
            name: name.to_string(),
            description: options.description.unwrap_or_default(),
            reference: options.reference,
            meta,
        };

        self.project_repo.create(&mut project).await?;
        self.event_bus.emit_file_created(&project).await?;
        Ok(project)
    }

    pub async fn update(
        &self,
        id: &str,
        name: &str,
        description: &str,
        created_by: &str,
    ) -> Result<Project> {
        info!("processing a \"update\" project request for user {created_by}",);

        let mut project = self.project_repo.find(id, created_by).await?;
        project.description = description.to_string();
        project.name = name.to_string();

        self.project_repo.update(&project).await?;
        Ok(project)
    }
}
