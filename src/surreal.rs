//! Surrealdb utilities for managing query responses and error handling.

use serde::de::DeserializeOwned;
use surrealdb::{error, Response};

use crate::result::{Error, Result};

impl From<error::Db> for Error {
    fn from(value: error::Db) -> Self {
        error!("{} parsing surrealdb sql query {}", Error::Unknown, value);
        Error::Unknown
    }
}

/// Given a query response and an statement's index, returns the content of that statement deserialized
/// into an instance of U.
pub fn export_item<T: DeserializeOwned, U: From<T>>(mut resp: Response, index: usize) -> Result<U> {
    resp.take::<Option<T>>(index)
        .map_err(|err| {
            error!(
                "{} taking item from statement {}: {}",
                Error::Unknown,
                index,
                err
            );

            Error::Unknown
        })?
        .map(Into::into)
        .ok_or(Error::NotFound)
}
