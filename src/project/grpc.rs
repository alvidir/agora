//! Infrastructure layer for serving the project's aplication as an gRPC service.

use crate::grpc;
use crate::project::{
    application::{ProjectApplication, ProjectRepository},
    domain,
};
use tonic::{Request, Response, Status};

// Import the generated rust code into module
mod proto {
    tonic::include_proto!("project");
}

// Proto generated server traits
use proto::project_service_server::ProjectService;
pub use proto::project_service_server::ProjectServiceServer;

// Proto message structs
use proto::{Empty, Project, ProjectList};

use super::application::{CreateOptions, EventBus};

pub struct GrpcProjectServer<P: ProjectRepository + Sync + Send, B: EventBus + Sync + Send> {
    pub project_app: ProjectApplication<P, B>,
    pub uid_header: &'static str,
}

#[tonic::async_trait]
impl<P: 'static + ProjectRepository + Sync + Send, B: 'static + EventBus + Sync + Send>
    ProjectService for GrpcProjectServer<P, B>
{
    async fn get(&self, request: Request<Project>) -> Result<Response<Project>, Status> {
        let uid = grpc::get_header(&request, self.uid_header)?;
        let msg_ref = request.into_inner();

        self.project_app
            .get(&msg_ref.id, &uid)
            .await
            .map(|projects| Response::new(projects.into()))
            .map_err(Into::into)
    }

    async fn list(&self, request: Request<Empty>) -> Result<Response<ProjectList>, Status> {
        let uid = grpc::get_header(&request, self.uid_header)?;

        self.project_app
            .list(&uid)
            .await
            .map(|projects| Response::new(projects.into()))
            .map_err(Into::into)
    }

    async fn create(&self, request: Request<Project>) -> Result<Response<Project>, Status> {
        let uid = grpc::get_header(&request, self.uid_header)?;
        let msg_ref = request.into_inner();

        self.project_app
            .create(
                &msg_ref.name,
                &uid,
                CreateOptions {
                    description: Some(msg_ref.description.to_string()),
                    ..Default::default()
                },
            )
            .await
            .map(|project| Response::new(project.into()))
            .map_err(Into::into)
    }

    async fn update(&self, request: Request<Project>) -> Result<Response<Project>, Status> {
        let uid = grpc::get_header(&request, self.uid_header)?;
        let msg_ref = request.into_inner();

        self.project_app
            .update(&msg_ref.id, &msg_ref.name, &msg_ref.description, &uid)
            .await
            .map(|project| Response::new(project.into()))
            .map_err(Into::into)
    }
}

impl From<domain::Project> for Project {
    fn from(value: domain::Project) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<Vec<domain::Project>> for ProjectList {
    fn from(value: Vec<domain::Project>) -> Self {
        Self {
            projects: value
                .into_iter()
                .map(|project| Project {
                    id: project.id,
                    name: project.name,
                    description: project.description,
                })
                .collect(),
        }
    }
}
