use anyhow::{Ok, Result};
use rust_web_warp_learning::socket_addr;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    let hello = warp::get().map(|| "Hello, World!".to_string());

    warp::serve(hello).run(socket_addr()).await;

    Ok(())
}
