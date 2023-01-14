//! Infrastructure layer for serving the project's aplication as an gRPC service.

use crate::project::application::{ProjectApplication, ProjectRepository};
use tonic::{Request, Response, Status};

// Import the generated rust code into module
mod proto {
    tonic::include_proto!("project");
}

// Proto generated server traits
use proto::project_server::Project;
pub use proto::project_server::ProjectServer;

// Proto message structs
use proto::ProjectDescriptor;

pub struct ProjectImplementation<P: ProjectRepository + Sync + Send> {
    pub project_app: ProjectApplication<P>,
    pub uid_header: &'static str,
}

#[tonic::async_trait]
impl<P: 'static + ProjectRepository + Sync + Send> Project for ProjectImplementation<P> {
    async fn create(
        &self,
        request: Request<ProjectDescriptor>,
    ) -> Result<Response<ProjectDescriptor>, Status> {
        unimplemented!()
    }
}
