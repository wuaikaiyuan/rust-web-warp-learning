use crate::{
    handler_error::handler::Error,
    model::question::{Question, QuestionId},
};
use anyhow::Result;

pub async fn get_question() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId("1".to_string()),
        "title".to_string(),
        "content".to_string(),
        None,
    );

    match question.id.0.parse::<i32>() {
        Ok(_) => Ok(warp::reply::json(&question)),
        Err(err) => Err(warp::reject::custom(Error::ParseError(err))),
    }
}
