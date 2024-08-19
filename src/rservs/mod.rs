pub mod master;
pub mod session;

struct Server {
    sessions: Vec<(session::Client, std::sync::mpsc::Sender<bool>)>,
    listener: (master::Listener, std::sync::mpsc::Sender<bool>, std::sync::mpsc::Receiver<std::net::TcpStream>),
}

impl Server {

    pub fn new() -> Self {
        Server { 
            sessions: Vec::new(),
            listener: master::Listener::new(port, start_receiver, session_streams_transmitter)
        }
    }
}