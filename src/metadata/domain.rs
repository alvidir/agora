//! Domain layer of the metadata entity

use std::time::SystemTime;

#[derive(Clone, Default)]
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

    pub fn created_by(&self) -> Option<&str> {
        self.created_by.as_deref()
    }

    pub fn created_at(&self) -> Option<SystemTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<SystemTime> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<SystemTime> {
        self.deleted_at
    }
}
