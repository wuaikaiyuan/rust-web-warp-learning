use std::{io::{Read, Write}, net::TcpStream, thread};

fn main() {
    let client = thread::spawn(|| {
        tcp_client(address().as_str());
    });

    client.join().unwrap();
}

fn tcp_client(addr: &str) {
    let mut stream = 
        TcpStream::connect(addr).unwrap();
    
    stream.write("hello".as_bytes()).unwrap();
    stream.flush().unwrap();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // println!("client received: {}", String::from_utf8_lossy(&buffer));
}

fn address() -> String {
    "127.0.0.1:8080".to_string()
}