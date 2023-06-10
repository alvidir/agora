//! Infrastructure layer for managing projects persistency on SurrealDB.

use super::{
    application::ProjectRepository,
    domain::{Cardinalities, Project, ProjectWithCardinalities},
};
use crate::metadata::repository::SurrealMetadata;
use crate::result::{Error, Result};
use crate::surreal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

const TABLENAME: &str = "project";

const QUERY_FIND_PROJECT: &str =
    "SELECT * FROM project WHERE id = $id AND meta.created_by = $created_by;";

const QUERY_FIND_ALL_PROJECTS_WITH_CARDINALITIES: &str = "SELECT *,
count(project.characters) AS total_characters,
count(project.objects) AS total_objects,
count(project.locations) AS total_locations,
count(project.events) AS total_events
FROM project
WHERE meta.created_by = $created_by;";

#[derive(Serialize, Deserialize, Debug)]
struct SurrealProject<'a> {
    id: Thing,
    name: Cow<'a, str>,
    description: Cow<'a, str>,
    reference: Option<Cow<'a, str>>,
    meta: Cow<'a, SurrealMetadata<'a>>,
    highlight: bool,
}

impl<'a> From<SurrealProject<'a>> for Project {
    fn from(value: SurrealProject<'a>) -> Self {
        Project {
            id: value.id.to_string(),
            name: value.name.into(),
            description: value.description.into(),
            reference: value.reference.map(Into::into),
            meta: value.meta.into_owned().into(),
            highlight: value.highlight,
        }
    }
}

impl<'a> From<&Project> for SurrealProject<'a> {
    fn from(value: &Project) -> Self {
        let metadata: SurrealMetadata<'a> = value.meta.clone().into();

        SurrealProject {
            id: Thing::from(value.id().as_bytes().to_vec()),
            name: value.name.clone().into(),
            description: value.description.clone().into(),
            reference: value.reference.clone().map(Into::into),
            meta: Cow::Owned(metadata),
            highlight: value.highlight,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SurrealProjectWithCardinalities<'a> {
    id: Thing,
    name: Cow<'a, str>,
    description: Cow<'a, str>,
    reference: Option<Cow<'a, str>>,
    meta: SurrealMetadata<'a>,
    highlight: bool,

    #[serde(skip_serializing)]
    total_characters: i32,
    #[serde(skip_serializing)]
    total_objects: i32,
    #[serde(skip_serializing)]
    total_locations: i32,
    #[serde(skip_serializing)]
    total_events: i32,
}

impl<'a> From<SurrealProjectWithCardinalities<'a>> for ProjectWithCardinalities {
    fn from(value: SurrealProjectWithCardinalities<'a>) -> Self {
        Self {
            project: Project {
                id: value.id.to_string(),
                name: value.name.into(),
                description: value.description.into(),
                reference: value.reference.map(Into::into),
                highlight: value.highlight,
                meta: value.meta.into(),
            },

            cardinalities: Cardinalities {
                total_characters: value.total_characters,
                total_objects: value.total_objects,
                total_locations: value.total_locations,
                total_events: value.total_events,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SurrealAnonymousProject<'a> {
    name: Cow<'a, str>,
    description: Cow<'a, str>,
    reference: Option<Cow<'a, str>>,
    meta: SurrealMetadata<'a>,
    highlight: bool,
}

impl<'a> From<&Project> for SurrealAnonymousProject<'a> {
    fn from(value: &Project) -> Self {
        SurrealAnonymousProject {
            name: value.name.clone().into(),
            description: value.description.clone().into(),
            reference: value.reference.clone().map(Into::into),
            meta: value.meta.clone().into(),
            highlight: value.highlight,
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
                    "{} performing select query by created_by and id on surreal: {}",
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

    async fn find_all(&self, created_by: &str) -> Result<Vec<ProjectWithCardinalities>> {
        let resp = self
            .client
            .query(QUERY_FIND_ALL_PROJECTS_WITH_CARDINALITIES)
            .bind(("created_by", created_by))
            .await
            .map_err(|err| {
                error!(
                    "{} performing select query by created_by and name on surreal: {}",
                    Error::Unknown,
                    err
                );

                Error::Unknown
            })?;

        Ok(surreal::export_items::<
            SurrealProjectWithCardinalities,
            ProjectWithCardinalities,
        >(resp, 0)?)
    }

    async fn create(&self, project: &mut Project) -> Result<()> {
        let created: SurrealProject = self
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

        project.id = created.id.to_string();
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
