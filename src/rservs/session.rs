use std::thread::JoinHandle;

pub struct Client {
    session_thread: std::thread::JoinHandle<()>,
    socket: std::net::TcpStream,
}

impl Client {
    pub fn run(valid_tcp_connection: std::net::TcpStream) -> (JoinHandle<()>, Client) {
        (
            std::thread::spawn(move || {
            Client::handle_connection();}),
            
            Client {
                session_thread: std::thread::spawn(move || {
                    Client::handle_connection();
                }),
                socket: valid_tcp_connection,
            }
        )
    }

    fn handle_connection() {
        
    }
}