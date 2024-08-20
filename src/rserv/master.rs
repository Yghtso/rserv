pub struct Listener {
    pub listener_thread: Option<std::thread::JoinHandle<()>>,
    pub listener: Option<std::sync::Arc<std::sync::Mutex<std::net::TcpListener>>>,
}

impl Listener {

pub fn new(addr: std::net::SocketAddr, listener_control_rx: std::sync::mpsc::Receiver<bool>, sessions_tx: std::sync::mpsc::Sender<std::net::TcpStream>) -> Listener {
        
        let self_host = addr;  
        let sock: std::sync::Arc<std::sync::Mutex<std::net::TcpListener>> = 
            std::sync::Arc::new(
            std::sync::Mutex::new(
            std::net::TcpListener::bind(self_host).expect("[ - ] [LISTENER] Cannot bind the given address to the server")));
        let sock_clone = std::sync::Arc::clone(&sock);
        
        let handle: std::thread::JoinHandle<()> = std::thread::Builder::new()
        .name("[LISTENER]".into())
        .spawn(move || {
            while listener_control_rx.recv().expect("[ - ] [LISTENER] Error starting the [LISTENER] thread") {}
            
            loop {
                let sock: std::sync::MutexGuard<std::net::TcpListener> = sock_clone.lock().unwrap();

                match sock.accept() {
                    Ok((socket, _addr)) => {
                        sessions_tx.send(socket).expect("[ - ] Error, the client socket can't be trasmitted to the main thread");
                    }
                    Err(e) => println!("[ - ] Error getting the client connection: {e:?}"),
                }
                std::mem::drop(sock);
                
            }
        }).expect("[ - ] Error the listener thread can' t be created");

        Listener {
            listener_thread: Some(handle),
            listener: Some(sock),
        }
    }
}