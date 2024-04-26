use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod handler_error;
pub mod model;
pub mod routes;

pub fn address() -> String {
    "127.0.0.1:8080".to_string()
}

pub fn socket_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
}
