//! Domain layer of the metadata entity

use std::time::SystemTime;

#[derive(Clone)]
pub struct Metadata {
    pub(super) created_by: Option<String>,
    pub(super) created_at: Option<SystemTime>,
    pub(super) updated_at: Option<SystemTime>,
    pub(super) deleted_at: Option<SystemTime>,
}

impl Metadata {
    pub fn new(created_by: &str) -> Self {
        Metadata {
            created_by: Some(created_by.to_string()),
            created_at: Some(SystemTime::now()),
            updated_at: None,
            deleted_at: None,
        }
    }
}
