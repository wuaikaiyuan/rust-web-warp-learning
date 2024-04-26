use anyhow::Result;
use warp::{
    filters::cors::CorsForbidden,
    reject::{Reject, Rejection},
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParamError,
    ResourceNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Parameters parse int err: {}", err)
            }
            Error::MissingParamError => write!(f, "Parameters is missing"),
            Error::ResourceNotFound => write!(f, "Resource not found"),
        }
    }
}

impl Reject for Error {}

pub async fn return_error(
    r: Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(err) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            err.to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
