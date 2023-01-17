//! Event payloads and datastructures

use serde::{Deserialize, Serialize};

#[derive(strum_macros::Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum EventKind {
    Created,
    Deleted,
}

/// Represents
#[derive(Serialize, Deserialize)]
pub struct FileEventPayload<'a> {
    pub(super) issuer: &'a str,
    pub(super) user_id: i32,
    pub(super) app_id: &'a str,
    pub(super) file_name: &'a str,
    pub(super) file_id: &'a str,
    pub(super) kind: EventKind,
}
