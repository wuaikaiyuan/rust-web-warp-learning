use anyhow::Result;
use rust_web_warp_learning::{
    handler_error::return_error, routes::get_question, socket_addr,
};
use warp::Filter;

async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    let our_ids = vec![1, 3, 7, 13];
    Ok(warp::reply::json(&our_ids))
}

#[tokio::main]
async fn main() -> Result<()> {
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(hello);

    let get_item = warp::get()
        .and(warp::path("question"))
        .and(warp::path::end())
        .and_then(get_question);

    let routes = get_item.or(hello).recover(return_error);

    warp::serve(routes).run(socket_addr()).await;

    Ok(())
}
