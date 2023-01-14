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

/// Given a query response and a field name the value of that field if, and only if, it exists
/// in the response. Otherwise an error is thrown.
pub fn export_field<T: DeserializeOwned>(resp: &mut Response, field: &str) -> Result<T> {
    let Some(value): Option<T> = resp.take(field).map_err(|err| {
        error!("{} getting {} from query result: {}", Error::Unknown, field, err);
        Error::Unknown
    })? else {
        error!("{} getting {} from query result: no value provided", Error::Unknown, field);
        return Err(Error::Unknown);
    };

    Ok(value)
}
