pub struct Listener {
    // thread that shares a socket Arc(Atomic reference count)
    pub listener_thread: Option<std::thread::JoinHandle<()>>,
    // socket that listens for income connections
    pub listener: Option<std::sync::Arc<std::sync::Mutex<std::net::TcpListener>>>,
}

impl Listener {

    // arguments : port;on wich the server star   -   receiver;that gets the signal to start the thread   -   transmitter; to send back the income connection
    // returns : Listener; istance of type Listener
    pub fn new(port: u16, start_receiver: std::sync::mpsc::Receiver<bool>, session_streams_transmitter: std::sync::mpsc::Sender<std::net::TcpStream>) -> Listener {
        
        let self_host: String = format!("127.0.0.1:{}", port);  
        let sock: std::sync::Arc<std::sync::Mutex<std::net::TcpListener>> = 
            std::sync::Arc::new(
            std::sync::Mutex::new(
            std::net::TcpListener::bind(self_host.as_str()).expect("Error in the initialization of the listener thread")));
        let sock_clone = std::sync::Arc::clone(&sock);
        

        let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            start_receiver.recv().unwrap();

            loop {
                let sock: std::sync::MutexGuard<std::net::TcpListener> = sock_clone.lock().unwrap();

                match sock.accept() {
                    Ok((socket, _addr)) => {
                        session_streams_transmitter.send(socket).expect("Error, the socket can't be trasmitted to the main thread");
                    }
                    Err(e) => println!("Error getting the client: {e:?}"),
                }
                // dropped explicitly the lock to be able to read 
                // from the socket by the main thread
                std::mem::drop(sock);
                
            }
        });

        Listener {
            listener_thread: Some(handle),
            listener: Some(sock),
        }
    }
}