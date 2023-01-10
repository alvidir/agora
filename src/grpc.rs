use tonic::{Request, Status};

use crate::errors;

pub fn get_header<T>(req: &Request<T>, header: &str) -> Result<String, Status> {
    let data = req
        .metadata()
        .get(header)
        .ok_or_else(|| Status::aborted(errors::ERR_NOT_FOUND))
        .map(|data| data.to_str())?;

    data.map(|data| data.to_string()).map_err(|err| {
        warn!(
            "{} parsing header data to str: {}",
            errors::ERR_INVALID_HEADER,
            err
        );
        Status::aborted(errors::ERR_INVALID_HEADER)
    })
}
