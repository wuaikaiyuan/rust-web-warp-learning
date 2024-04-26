use anyhow::Result;
use warp::{
    filters::cors::CorsForbidden,
    reject::{Reject, Rejection},
};

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

pub async fn return_error(
    r: Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if r.find::<InvalidId>().is_some() {
        Ok(warp::reply::with_status(
            "No valid id provided".to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
