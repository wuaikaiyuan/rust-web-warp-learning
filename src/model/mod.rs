use anyhow::Result;
use serde::{Deserialize, Serialize};
use warp::{
    filters::cors::CorsForbidden,
    reject::{Reject, Rejection},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId(String);

impl Question {
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::str::FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "question id can not be empty",
            )),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

pub async fn get_question() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId("1".to_string()),
        "title".to_string(),
        "content".to_string(),
        None,
    );

    match question.id.0.parse::<i32>() {
        Ok(_) => Ok(warp::reply::json(&question)),
        Err(_) => Err(warp::reject::custom(InvalidId)),
    }
}

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
