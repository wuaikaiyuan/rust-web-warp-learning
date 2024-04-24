use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

fn main() {
    let server = thread::spawn(|| {
        tcp_server(address().as_str()).unwrap();
    });

    server.join().unwrap();
}

fn tcp_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 5];

    stream.read_exact(&mut buffer).unwrap();
    println!("server received: {}", String::from_utf8_lossy(&buffer));

    let response = "ni hao!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}

fn address() -> String {
    "127.0.0.1:8080".to_string()
}