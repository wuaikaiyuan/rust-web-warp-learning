use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() {
    let address = rust_web_warp_learning::address();
    // 创建一个 TcpListener 并绑定到指定地址和端口
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server>> running on 127.0.0.1:8080");

    loop {
        // 接受来自客户端的连接
        let (socket, _) = listener.accept().await.unwrap();

        // 使用 LengthDelimitedCodec 来处理长度分隔的消息
        let framed = Framed::new(socket, LengthDelimitedCodec::new());

        // 每个客户端连接创建一个异步任务处理
        tokio::spawn(async move {
            // 处理客户端连接中的消息
            if let Err(e) = handle_client(framed).await {
                eprintln!("Server>> Error handling client: {}", e);
            }
        });
    }
}

// 处理客户端发送的异步任务
async fn handle_client(
    mut framed: Framed<tokio::net::TcpStream, LengthDelimitedCodec>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 循环接收和处理消息
    while let Some(msg) = framed.next().await {
        let bytes = msg?;

        // 模拟消息处理：打印消息
        let received_message = String::from_utf8_lossy(&bytes);
        println!("Server>> Received message: {}", received_message);

        let send_message = "Pong";
        // 将消息发送回客户端
        framed
            .send(tokio_util::bytes::Bytes::from(send_message))
            .await
            .unwrap();
        println!("Server>> Send message to client: {}", send_message);
    }

    Ok(())
}
