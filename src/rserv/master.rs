pub struct Listener {
    pub listener_thread: std::thread::JoinHandle<()>,
    pub listener: std::sync::Arc<std::sync::Mutex<std::net::TcpListener>>,
    pub status: std::sync::Arc<std::sync::Mutex<super::ThreadStatus>>,
}

impl Listener {

pub fn new(addr: std::net::SocketAddr, listener_control_rx: std::sync::mpsc::Receiver<bool>, sessions_tx: std::sync::mpsc::Sender<std::net::TcpStream>) -> Listener {
          
        let sock = 
            std::sync::Arc::new(
            std::sync::Mutex::new(
            std::net::TcpListener::bind(addr).expect("[ - ] [LISTENER] Cannot bind the given address to the server")));

        let status = 
            std::sync::Arc::new(
            std::sync::Mutex::new(
            super::ThreadStatus::Blocked));
        
        // clone for the Arc move in the thread to own them
        let sock_clone = std::sync::Arc::clone(&sock);
        let status_clone = std::sync::Arc::clone(&status);
        
        let handle: std::thread::JoinHandle<()> = std::thread::Builder::new()
        .name("[LISTENER]".into())
        .spawn(move || {
            loop { 
                // TODO : error handling 
                let result = listener_control_rx.try_recv();
                let change_state = match result {
                    Ok(signal) => signal,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        println!("[ - ] [MAIN] Error, the comunication between [MAIN] and [LISTENER] threads is interrupted");
                        false
                    }
                    _ => false, 
                };
                    
                let mut status = status_clone.lock().unwrap();
                match *status {

                    super::ThreadStatus::Blocked => { if change_state {*status = super::ThreadStatus::Running}},
                    super::ThreadStatus::Running => {
                        
                        let sock: std::sync::MutexGuard<std::net::TcpListener> = sock_clone.lock().unwrap();

                        match sock.accept() {
                            Ok((socket, _addr)) => {
                                sessions_tx.send(socket).expect("[ - ] [LISTENER] Error, the client socket can't be trasmitted to the main thread");
                            }
                            Err(e) => println!("[ - ] [LISTENER] Error getting the client connection: {e:?}"),
                        }
                        if change_state {*status = super::ThreadStatus::Blocked};
                    }
                }
            }
        }).expect("[ - ] [MAIN] Error the [LISTENER] thread can' t be created");

        Listener {
            listener_thread: handle,
            listener: sock,
            status: status,
        }
    }
}