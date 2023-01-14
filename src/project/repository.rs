//! Infrastructure layer for managing projects persistency on SurrealDB.

use std::borrow::Cow;

use super::{application::ProjectRepository, domain::Project};
use crate::result::{Error, Result};
use crate::surreal;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
use surrealdb::{sql, Response};

const TABLENAME: &str = "project";

#[derive(Serialize, Deserialize)]
struct SurrealProject<'a> {
    id: Option<String>,
    user_id: Cow<'a, str>,
    name: Cow<'a, str>,
}

/// Repository for managing projects persistency
pub struct SurrealProjectRepository<'a> {
    pub client: &'a Surreal<Client>,
}

impl<'a> SurrealProjectRepository<'a> {
    fn build(mut resp: Response) -> Result<Project> {
        Ok(Project {
            id: surreal::export_field(&mut resp, "id")?,
            name: surreal::export_field(&mut resp, "name")?,
            user_id: surreal::export_field(&mut resp, "user_id")?,
        })
    }
}

#[async_trait]
impl<'a> ProjectRepository for SurrealProjectRepository<'a> {
    async fn find_by_name(&self, name: &str) -> Result<Project> {
        let sql = sql! {
            SELECT *
            FROM type::table($table)
            WHERE name = type::string($name)
        };

        let resp = self
            .client
            .query(sql)
            .bind(("table", TABLENAME))
            .bind(("name", name))
            .await
            .map_err(|err| {
                error!(
                    "{} performing select by name query on surreal: {:?}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        if resp.num_statements() == 0 {
            return Err(Error::NotFound);
        }

        Self::build(resp)
    }

    async fn create(&self, project: &mut Project) -> Result<()> {
        let surreal_project: SurrealProject = self
            .client
            .create("project")
            .content(SurrealProject {
                id: None,
                user_id: project.user_id.clone().into(),
                name: project.name.clone().into(),
            })
            .await
            .map_err(|err| {
                error!(
                    "{} performing create query on surreal: {:?}",
                    Error::Unknown,
                    err
                );
                Error::Unknown
            })?;

        if let Some(id) = surreal_project.id {
            project.id = id;
        }

        Ok(())
    }
}
