use std::collections::HashMap;

use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", res);

    Ok(())
}
