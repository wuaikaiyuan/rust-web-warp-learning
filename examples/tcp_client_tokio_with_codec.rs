use anyhow::Result;
use futures::prelude::*;
use rust_web_warp_learning::address;
use std::error::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = address();
    tcp_client_with_codec(&address).await?;

    Ok(())
}

// 异步 TCP 客户端
#[warn(dead_code)]
async fn _tcp_client(address: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address).await?;
    stream.write_all("src".as_bytes()).await?;
    println!("Message sent to server");

    // 关闭连接
    stream.flush().await.unwrap();
    println!("Connection closed");
    Ok(())
}

// 异步 TCP 客户端（带编码器）
async fn tcp_client_with_codec(
    address: &str,
) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect(address).await?;

    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    // 发送一条消息到服务器
    let message = "Hello, server!";
    framed.send(message.into()).await.unwrap();
    println!("Client>> Send message to server: {}", message);

    // 循环接收来自服务器的响应
    loop {
        // 读取一条来自服务器的消息
        let msg = framed.next().await.unwrap().unwrap();

        // 处理消息，这里只是简单地将收到的消息打印出来
        let received_message = String::from_utf8_lossy(&msg);
        println!(
            "Client>> Received message from server: {}",
            received_message
        );
    }
}
