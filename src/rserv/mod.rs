pub mod master;
pub mod session;

pub struct Server {
    sessions: Vec<(session::Client, std::sync::mpsc::Sender<bool>)>,
    listener: Option<master::Listener>,
    listener_control_tx: Option<std::sync::mpsc::Sender<bool>>,
    sessions_rx: Option<std::sync::mpsc::Receiver<std::net::TcpStream>>,
}

impl Server {

    pub fn new() -> Self {
        Server {
            sessions: Vec::new(),
            listener: None,
            listener_control_tx: None,
            sessions_rx: None,
        }
    }

    pub fn bind(mut self, address: std::net::SocketAddr) -> Self {
        let (listener_control_tx, listener_control_rx) = std::sync::mpsc::channel::<bool>();
        self.listener_control_tx = Some(listener_control_tx);

        let (sessions_tx, sessions_rx) = std::sync::mpsc::channel::<std::net::TcpStream>();
        self.sessions_rx = Some(sessions_rx);

        self.listener = Some(master::Listener::new(address, listener_control_rx, sessions_tx));

        self
    }

    pub fn run(mut self) -> Option<Self>{
        if self.listener.is_none() || self.listener_control_tx.is_none() || self.sessions_rx.is_none() {
            println!("[ - ] [MAIN] Error, the server was trying to start but some necessary fields weren' t initialized");
            return None;
        }

        println!("[ + ] [MAIN] The server started correctly");

        self.listener_control_tx.as_ref().unwrap().send(true).expect("[ - ] [MAIN] Error, the [LISTENER] thread can't be started");
        
        loop {
            let result = self.sessions_rx.as_ref().unwrap().try_recv();
            match result {
                Ok(socket) => {
                    let (session_control_tx, session_control_rx) = std::sync::mpsc::channel::<bool>();
                    let client: session::Client = session::Client::new(socket, session_control_rx);
                    self.sessions.push((client, session_control_tx));
                    println!("[ + ] [MAIN] Spawnato un nuovo client");
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    println!("[ - ] [MAIN] Error, the comunication between [MAIN] and [LISTENER] threads is interrupted");
                }
                _ => ()
            }
        }
        
        Some(self)
    }
}

enum ThreadStatus {

    // Actively executing task
    Running, 

    // Execution paused 
    Blocked,

}