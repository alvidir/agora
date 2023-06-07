//! Infrastructure layer for managing metadata persistency on SurrealDB.

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, time::SystemTime};

use super::domain::Metadata;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SurrealMetadata<'a> {
    pub(super) created_by: Option<Cow<'a, str>>,
    pub(super) created_at: Option<Cow<'a, SystemTime>>,
    pub(super) updated_at: Option<Cow<'a, SystemTime>>,
    pub(super) deleted_at: Option<Cow<'a, SystemTime>>,
}

impl<'a> From<Metadata> for SurrealMetadata<'a> {
    fn from(value: Metadata) -> Self {
        SurrealMetadata {
            created_by: value.created_by.map(Into::into),
            created_at: value.created_at.map(Cow::Owned),
            updated_at: value.updated_at.map(Cow::Owned),
            deleted_at: value.deleted_at.map(Cow::Owned),
        }
    }
}

impl<'a> From<SurrealMetadata<'a>> for Metadata {
    fn from(value: SurrealMetadata<'a>) -> Self {
        Metadata {
            created_by: value.created_by.map(Into::into),
            created_at: value.created_at.map(|item| item.into_owned()),
            updated_at: value.updated_at.map(|item| item.into_owned()),
            deleted_at: value.deleted_at.map(|item| item.into_owned()),
        }
    }
}
