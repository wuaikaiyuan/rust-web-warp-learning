#![warn(clippy::all)]
use std::sync::{Arc, Mutex};

use anyhow::Result;
use rust_web_warp_learning::{
    dao::DataBase,
    handler_error::handler::return_error,
    routes::question::{
        add_question, delete_question, get_question_by_id,
        get_question_by_pagination, update_question,
    },
    socket_addr, AppState,
};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    let our_ids = vec![1, 3, 7, 13];
    Ok(warp::reply::json(&our_ids))
}

#[tokio::main]
async fn main() -> Result<()> {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        "practical_rust_book=info,warp=info".to_owned()
    });

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let trace_filter = warp::trace(|info| {
        tracing::info_span!(
            "当前请求：",
            method = %info.method(),
            path = %info.path(),
            trace_id = %uuid::Uuid::new_v4(),
            from = %info.remote_addr().unwrap(),
        )
    });

    // 跨域设置
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]);

    // 加载.env配置
    dotenv::dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::new(DataBase::new(&db_url).await);
    let app_state = warp::any().map(move || {
        Arc::new(AppState {
            context: db.clone(),
            connections: Mutex::new(0),
        })
    });

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(hello);

    // GET /question?limit=10&offset=0
    let get_question_by_pagination = warp::get()
        .and(warp::path("question"))
        .and(warp::path::end())
        .and(warp::query())
        .and(app_state.clone())
        .and_then(get_question_by_pagination);

    // GET /question/{id}
    let get_question_by_id = warp::get()
        .and(warp::path("question"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(app_state.clone())
        .and_then(get_question_by_id);

    // POST /question
    // {"title": "hello", "content": "world", "tags": "tag1,tag2"}
    let add_question = warp::post()
        .and(warp::path("question"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(app_state.clone())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("question"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(app_state.clone())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("question"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(app_state.clone())
        .and_then(delete_question);

    let routes = get_question_by_id
        .or(hello)
        .or(get_question_by_pagination)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .with(warp::trace::request())
        .with(trace_filter)
        .recover(return_error);

    warp::serve(routes).run(socket_addr()).await;

    Ok(())
}
