use rust_web_warp_learning::socket_addr;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("1 - {}", name));
    warp::serve(hello).run(socket_addr()).await;
}
