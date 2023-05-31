//! Domain layer of the project entity.

use crate::metadata::domain::Metadata;

/// Represents an agora's project
pub struct Project {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) reference: Option<String>,
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
