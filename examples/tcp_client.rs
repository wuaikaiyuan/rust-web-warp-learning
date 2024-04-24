use rust_web_warp_learning::address;
use std::{io::Write, net::TcpStream};

fn main() {
    tcp_client(address().as_str());
}

fn tcp_client(addr: &str) {
    let mut stream = TcpStream::connect(addr).unwrap();

    stream.write_all("&[1,2,3]".as_bytes()).unwrap();
    stream.write_all("&[4,5,6]".as_bytes()).unwrap();
    stream.write_all("&[7,8,9]".as_bytes()).unwrap();
    stream.flush().unwrap();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // println!("client received: {}", String::from_utf8_lossy(&buffer));
}
