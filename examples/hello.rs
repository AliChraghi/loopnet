use hoop;
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let s = hoop::HttpServer::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80, 2);
    s.listen();
}
