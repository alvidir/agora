//! Domain layer of the project entity.

/// Represents an agora's project
pub struct Project {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) name: String,
}

impl Project {
    pub fn new(id: String, user_id: String, name: String) -> Self {
        Self { id, user_id, name }
    }
}
