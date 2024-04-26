#![warn(clippy::all)]

use anyhow::Result;
use rust_web_warp_learning::{
    handler_error::handler::return_error,
    routes::question::{get_question, get_question_by_params},
    socket_addr,
};
use warp::{http::Method, Filter};

async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    let our_ids = vec![1, 3, 7, 13];
    Ok(warp::reply::json(&our_ids))
}

#[tokio::main]
async fn main() -> Result<()> {
    let _log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        "practical_rust_book=info,warp=info".to_owned()
    });

    // 初始化日志
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("log4rs init success");
    log::info!("warp server is starting ... ");

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

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(hello);

    // GET /question
    let get_item_by_params = warp::get()
        .and(warp::path("question"))
        .and(warp::path::end())
        .and(warp::query())
        .and_then(get_question_by_params);

    // GET /question/{id}
    let get_item = warp::get()
        .and(warp::path("question"))
        .and(warp::path::param::<i32>())
        .and(warp::query())
        .and(warp::path::end())
        .and_then(get_question);

    let routes = get_item
        .or(hello)
        .or(get_item_by_params)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(socket_addr()).await;

    Ok(())
}
