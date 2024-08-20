pub mod master;
pub mod session;

struct Server {
    sessions: Vec<(session::Client, std::sync::mpsc::Sender<bool>)>,
    listener: Option<master::Listener>,
    listener_control: Option<std::sync::mpsc::Sender<bool>>,
    sessions_reciever: Option<std::sync::mpsc::Receiver<std::net::TcpStream>>,
}

impl Server {

    pub fn new() -> Self {
        Server {
            sessions: Vec::new(),
            listener: None,
            listener_control: None,
            sessions_reciever: None,
        }
    }

    
}