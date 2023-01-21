//! Domain layer of the project entity.

use crate::metadata::domain::Metadata;

/// Represents an agora's project
pub struct Project {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) meta: Metadata,
}

impl Project {
    pub fn new(id: &str, name: &str, meta: Metadata) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            meta,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}
