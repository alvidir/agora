//! Infrastructure layer for managing projects persistency on SurrealDB.

use super::{application::ProjectRepository, domain::Project};
use crate::metadata::repository::SurrealMetadata;
use crate::result::{Error, Result};
use crate::surreal;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::sql;
use surrealdb::{engine::remote::ws::Client, Surreal};

const TABLENAME: &str = "project";

#[derive(Serialize, Deserialize, Debug)]
struct SurrealProject<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
    meta: Cow<'a, SurrealMetadata<'a>>,
}

impl<'a> From<SurrealProject<'a>> for Project {
    fn from(value: SurrealProject<'a>) -> Self {
        Project {
            id: value.id.into(),
            name: value.name.into(),
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
            meta: Cow::Owned(metadata),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SurrealNewProject<'a> {
    name: Cow<'a, str>,
    meta: SurrealMetadata<'a>,
}

impl<'a> From<&Project> for SurrealNewProject<'a> {
    fn from(value: &Project) -> Self {
        SurrealNewProject {
            name: value.name.clone().into(),
            meta: value.meta.clone().into(),
        }
    }
}

/// Repository for managing projects persistency
pub struct SurrealProjectRepository<'a> {
    pub client: &'a Surreal<Client>,
}

#[async_trait]
impl<'a> ProjectRepository for SurrealProjectRepository<'a> {
    async fn find_by_created_by_and_name(&self, created_by: &str, name: &str) -> Result<Project> {
        let sql = sql! {
            SELECT *
            FROM project
            WHERE name = $name AND meta.created_by = $created_by;
        };

        let resp = self
            .client
            .query(sql)
            .bind(("created_by", created_by))
            .bind(("name", name))
            .await
            .map_err(|err| {
                error!(
                    "{} performing select by created_by and name query on surreal: {}",
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

    async fn create(&self, project: &mut Project) -> Result<()> {
        let resp: SurrealProject = self
            .client
            .create(TABLENAME)
            .content(Into::<SurrealNewProject>::into(&*project))
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
}