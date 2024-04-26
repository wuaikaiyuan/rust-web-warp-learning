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
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("params: {:#?}", param);
    // let mut question = Question::default();
    // question.id.0 = id.to_string();

    let mut question = Question {
        id: QuestionId(id.to_string()),
        ..Default::default()
    };

    if !param.is_empty() {
        if param.contains_key("title") {
            question.title = param.get("title").unwrap().to_string();
        }

        if param.contains_key("content") {
            question.content = param.get("content").unwrap().to_string();
        }
        if param.contains_key("tags") {
            let tags = serde_json::from_str::<Vec<String>>(
                param.get("tags").unwrap(),
            )
            .map_err(|err| {
                warp::reject::custom(
                    crate::handler_error::handler::Error::SerdeJsonErr(
                        err,
                        String::from("[tags] parse error: invalid type"),
                    ),
                )
            })?;

            question.tags = Some(tags);
        } else {
            question.tags = None;
        }
    }

    Ok(warp::reply::json(&question))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_json() {
        let j = "[1,2,3]";

        let u: Vec<i32> = serde_json::from_str(j).unwrap();
        println!("{:#?}", u);
    }
}
