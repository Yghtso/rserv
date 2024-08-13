mod rservs;
use std::net::TcpListener;
fn main() {
    
    // let master_thread: rservs::master::Listener = rservs::master::Listener::run(80);

    // let _ = master_thread.listener_thread.join();

    let listener = TcpListener::bind("192.168.1.10:8080").unwrap();

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}