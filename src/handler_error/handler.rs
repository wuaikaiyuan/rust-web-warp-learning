use anyhow::Result;
use warp::{
    filters::cors::CorsForbidden,
    reject::{MethodNotAllowed, Reject, Rejection},
};

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    SerdeJsonErr(serde_json::Error, String),
    WarpCorsForbidden(CorsForbidden),
    WarpMethodNotAllowed(MethodNotAllowed),
    MissingParamError(String),
    ResourceNotFound,
    DatabaseError(sqlx::Error),
    InvalidLimit(String),
    InvalidOffset(String),
    UpdateError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Parameters parse err: {}", err)
            }
            Error::SerdeJsonErr(ref err, ref msg) => {
                write!(f, "Serde json err: {} \r\n msg: {}", err, msg)
            }
            Error::WarpCorsForbidden(ref err) => {
                write!(f, "Warp cors forbidden: {}", err)
            }
            Error::WarpMethodNotAllowed(ref err) => {
                write!(f, "Warp method not allowed: {}", err)
            }
            Error::MissingParamError(ref err) => {
                write!(f, "Parameters {} is missing", err)
            }
            Error::ResourceNotFound => write!(f, "Resource not found"),
            Error::DatabaseError(ref err) => {
                write!(f, "Database error: {}", err)
            }
            Error::InvalidLimit(ref err) => {
                write!(f, "Invalid limit: {}", err)
            }
            Error::InvalidOffset(ref err) => {
                write!(f, "Invalid offset: {}", err)
            }
            Error::UpdateError(ref err) => {
                write!(f, "Update error: {}", err)
            }
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
