use crate::model::{
    pagination::{extract_pagination, Pagination},
    question::{Question, QuestionId},
};
use anyhow::Result;
use std::collections::HashMap;

pub async fn get_question_by_params(
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // 设置默认参数
    let mut pagination = Pagination::default();

    if !param.is_empty() {
        pagination = extract_pagination(param)?;
    }

    Ok(warp::reply::json(&pagination))

    // Ok(warp::reply::with_status(
    //     "success".to_string(),
    //     warp::http::StatusCode::OK,
    // ))
}

// GET /question/{id}
pub async fn get_question(
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId(id.to_string()),
        "title".to_string(),
        "content".to_string(),
        None,
    );

    // match question.id.0.parse::<i32>() {
    //     Ok(_) => Ok(warp::reply::json(&question)),
    //     Err(err) =>
    // Err(warp::reject::custom(crate::handler_error::handler::Error::ParseError(err))),
    // }

    Ok(warp::reply::json(&question))
}
