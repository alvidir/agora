//! Infrastructure layer for managing projects persistency on SurrealDB.

use super::{application::ProjectRepository, domain::Project};
use crate::metadata::repository::SurrealMetadata;
use crate::result::{Error, Result};
use crate::surreal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::{engine::remote::ws::Client, Surreal};

const TABLENAME: &str = "project";

const QUERY_FIND_PROJECT: &str =
    "SELECT * FROM project WHERE id = $id AND meta.created_by = $created_by;";
const QUERY_FIND_ALL_PROJECTS: &str = "SELECT * FROM project WHERE meta.created_by = $created_by;";

#[derive(Serialize, Deserialize, Debug)]
struct SurrealProject<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    description: Cow<'a, str>,
    reference: Option<Cow<'a, str>>,
    meta: Cow<'a, SurrealMetadata<'a>>,
}

impl<'a> From<SurrealProject<'a>> for Project {
    fn from(value: SurrealProject<'a>) -> Self {
        Project {
            id: value.id.into(),
            name: value.name.into(),
            description: value.description.into(),
            reference: value.reference.map(Into::into),
            meta: value.meta.into_owned().into(),
        }
    }
}

impl<'a> From<&Project> for SurrealProject<'a> {
    fn from(value: &Project) -> Self {
        let metadata: SurrealMetadata<'a> = value.meta.clone().into();

        SurrealProject {
            id: value.id.clone().into(),
            name: value.name.clone().into(),
            description: value.description.clone().into(),
            reference: value.reference().map(ToString::to_string).map(Into::into),
            meta: Cow::Owned(metadata),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SurrealAnonymousProject<'a> {
    name: Cow<'a, str>,
    reference: Option<Cow<'a, str>>,
    meta: SurrealMetadata<'a>,
}

impl<'a> From<&Project> for SurrealAnonymousProject<'a> {
    fn from(value: &Project) -> Self {
        SurrealAnonymousProject {
            name: value.name.clone().into(),
            reference: value.reference().map(ToString::to_string).map(Into::into),
            meta: value.meta.clone().into(),
        }
    }
}

/// Repository for managing projects persistency
pub struct SurrealProjectRepository<'a> {
    pub client: &'a Surreal<Client>,
}

#[async_trait::async_trait]
impl<'a> ProjectRepository for SurrealProjectRepository<'a> {
    async fn find(&self, created_by: &str, id: &str) -> Result<Project> {
        let resp = self
            .client
            .query(QUERY_FIND_PROJECT)
            .bind(("created_by", created_by))
            .bind(("id", id))
            .await
            .map_err(|err| {
                error!(
                    "{} performing select by created_by and id query on surreal: {}",
                    Error::Unknown,
                    err
                );

                Error::Unknown
            })?;

        let item = surreal::export_item::<SurrealProject, Project>(resp, 0)?;
        if item.id.is_empty() {
            return Err(Error::NotFound);
        }

        Ok(item)
    }

    async fn find_all(&self, created_by: &str) -> Result<Vec<Project>> {
        let resp = self
            .client
            .query(QUERY_FIND_ALL_PROJECTS)
            .bind(("created_by", created_by))
            .await
            .map_err(|err| {
                error!(
                    "{} performing select by created_by and name query on surreal: {}",
                    Error::Unknown,
                    err
                );

                Error::Unknown
            })?;

        Ok(surreal::export_items::<SurrealProject, Project>(resp, 0)?)
    }

    async fn create(&self, project: &mut Project) -> Result<()> {
        let resp: SurrealProject = self
            .client
            .create(TABLENAME)
            .content(Into::<SurrealAnonymousProject>::into(&*project))
            .await
            .map_err(|err| {
                error!(
                    "{} performing create query on surreal: {}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        project.id = resp.id.into();
        Ok(())
    }

    async fn update(&self, project: &Project) -> Result<()> {
        self.client
            .update((TABLENAME, project.id()))
            .content(Into::<SurrealProject>::into(project))
            .await
            .map_err(|err| {
                error!(
                    "{} performing create query on surreal: {}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        Ok(())
    }
}
