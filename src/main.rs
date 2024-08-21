use std::net::{Ipv4Addr, SocketAddr};
mod rserv;
fn main() {

    let s = rserv::Server::new()
    .bind(SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 8080)))
    .run();

    match s {
        None => println!("[ - ] [MAIN] Error, the server couldn't be started"),
        _ => (),
    }
}