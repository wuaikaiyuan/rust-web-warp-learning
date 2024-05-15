use crate::{
    model::{extract_pagination, Pagination, Question},
    AppState,
};
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use tracing::{event, instrument, Level};

#[instrument]
pub async fn get_question_by_pagination(
    param: HashMap<String, String>,
    data: Arc<AppState<'_>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // log::info!("Start query question by params: {:#?}", param);
    event!(target: "practical warp query by pagination", Level::INFO, "get_question_by_pagination");
    // 设置默认参数
    let mut pagination = Pagination::default();

    if !param.is_empty() {
        pagination = extract_pagination(param)?;
    }

    match data
        .context
        .questions
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(data) => Ok(warp::reply::json(&data)),
        Err(e) => {
            event!(target: "practical warp query by pagination", Level::ERROR, "get_question_by_pagination error: {}", e);
            Err(warp::reject::custom(e))
        }
    }

    // Ok(warp::reply::with_status(
    //     "success".to_string(),
    //     warp::http::StatusCode::OK,
    // ))
}

// GET /question?id=1&title=hello&content=world&tags=tag1,tag2
#[instrument]
pub async fn get_question(
    id: i64,
    param: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "get question by id and params", Level::INFO, "get_question");

    let mut question = Question {
        id: Some(id),
        ..Default::default()
    };

    if !param.is_empty() {
        if param.contains_key("title") {
            question.title = param.get("title").unwrap().to_string();
        }

        if param.contains_key("content") {
            question.content =
                Some(param.get("content").unwrap().to_string());
        }
        if param.contains_key("tags") {
            // let tags = serde_json::from_str::<Vec<String>>(
            //     param.get("tags").unwrap(),
            // )
            // .map_err(|err| {
            //     warp::reject::custom(
            //         crate::handler_error::handler::Error::SerdeJsonErr(
            //             err,
            //             String::from("[tags] parse error: invalid type"),
            //         ),
            //     )
            // })?;

            question.tags = Some(param.get("tags").unwrap().to_string());
        } else {
            question.tags = None;
        }
    }

    Ok(warp::reply::json(&question))
}

// GET /question/{id}
#[instrument]
pub async fn get_question_by_id(
    id: i64,
    data: Arc<AppState<'_>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "get question by id", Level::INFO, "get_question_by_id");
    let question = data.context.questions.get_question_by_id(id).await;
    match question {
        Ok(q) => Ok(warp::reply::json(&q)),
        Err(e) => {
            event!(target: "get question by id", Level::ERROR, "get_question_by_id error: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}

#[instrument]
pub async fn add_question(
    question: Question,
    data: Arc<AppState<'_>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "add question", Level::INFO, "add_question params: {:#?}", question);
    match data.context.questions.add_question(&question).await {
        Ok(question) => {
            event!(target: "add_question controller ", Level::INFO, " return values : {:#?}", &question);
            Ok(warp::reply::json(&question))
        }
        Err(e) => {
            event!(target: "add question", Level::ERROR, "add_question error: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}

#[instrument]
pub async fn update_question(
    question: Question,
    data: Arc<AppState<'_>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "update question", Level::INFO, "update_question params: {:#?}", question);
    match data.context.questions.update_question(&question).await {
        Ok(question) => {
            event!(target: "update question", Level::INFO, "update_question return: {:#?}", question);
            Ok(warp::reply::json(&question))
        }
        Err(e) => {
            event!(target: "update question", Level::ERROR, "update_question error: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}

#[instrument]
pub async fn delete_question(
    id: i64,
    data: Arc<AppState<'_>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "delete question", Level::INFO, "delete_question params: {:#?}", id);
    match data.context.questions.delete_question_logic(id).await {
        Ok(_) => {
            event!(target: "delete question", Level::INFO, "delete_question return: success");
            Ok(warp::reply::json(&"success"))
        }
        Err(e) => {
            event!(target: "delete question", Level::ERROR, "delete_question error: {}", e);
            Err(warp::reject::custom(e))
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_json() {
        let j = "[1,2,3]";
        let u: Vec<i32> = serde_json::from_str(j).unwrap();
        assert_eq!(u, vec![1, 2, 3]);
    }
}
