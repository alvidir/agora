//! Infrastructure layer for serving the project's aplication as an gRPC service.

use crate::grpc;
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

pub struct GrpcProjectServer<P: ProjectRepository + Sync + Send> {
    pub project_app: ProjectApplication<P>,
    pub uid_header: &'static str,
}

#[tonic::async_trait]
impl<P: 'static + ProjectRepository + Sync + Send> Project for GrpcProjectServer<P> {
    async fn create(
        &self,
        request: Request<ProjectDescriptor>,
    ) -> Result<Response<ProjectDescriptor>, Status> {
        let uid = grpc::get_header(&request, self.uid_header)?;
        let msg_ref = request.into_inner();

        self.project_app
            .create("", &uid, &msg_ref.name)
            .await
            .map(|project| {
                Response::new(ProjectDescriptor {
                    id: project.id,
                    name: msg_ref.name,
                })
            })
            .map_err(Into::into)
    }
}
