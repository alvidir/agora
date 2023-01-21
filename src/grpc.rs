//! gRPC utilities for managing request's headers and conversions.

use tonic::{Request, Status};

use crate::result::Error;

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown => Status::unknown(value),
            Error::NotFound => Status::not_found(value),
            Error::NotAvailable => Status::unavailable(value),
            Error::Unauthorized => Status::permission_denied(value),
            Error::InvalidToken | Error::InvalidFormat | Error::InvalidHeader => {
                Status::invalid_argument(value)
            }
            Error::WrongCredentials => Status::unauthenticated(value),
            Error::RegexNotMatch => Status::failed_precondition(value),
            Error::AlreadyExists => Status::already_exists(value),
            Error::MissingFields => Status::invalid_argument(value),
        }
    }
}

/// Given a gPRC request, returns the value of the provided header's key if any, otherwise an error
/// is returned.
pub fn get_header<T>(req: &Request<T>, header: &str) -> Result<String, Status> {
    let data = req
        .metadata()
        .get(header)
        .ok_or_else(|| Into::<Status>::into(Error::Unauthorized))
        .map(|data| data.to_str())?;

    data.map(|data| data.to_string()).map_err(|err| {
        warn!(
            "{} parsing header data to str: {}",
            Error::InvalidHeader,
            err
        );
        Error::InvalidHeader.into()
    })
}
