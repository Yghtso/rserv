use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod rserv;
fn main() {

    let s: rserv::Server = rserv::Server::new()
    .bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 30000));
    
    s.run();
}