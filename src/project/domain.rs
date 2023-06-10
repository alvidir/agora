//! Domain layer of the project entity.

use crate::metadata::domain::Metadata;

/// Represents a project
#[derive(Debug)]
pub struct Project {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) reference: Option<String>,
    pub(super) highlight: bool,
    pub(super) meta: Metadata,
}

impl Project {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

/// Represents all the cardinalities a project could have
#[derive(Debug)]
pub struct Cardinalities {
    pub(super) total_characters: i32,
    pub(super) total_objects: i32,
    pub(super) total_locations: i32,
    pub(super) total_events: i32,
}

/// Represents a [`Project`] and all its [`Cardinalities`]
#[derive(Debug)]
pub struct ProjectWithCardinalities {
    pub project: Project,
    pub cardinalities: Cardinalities,
}
