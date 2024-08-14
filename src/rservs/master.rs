pub struct Listener {
    pub listener_thread: Option<std::thread::JoinHandle<()>>,
    pub socket: Option<std::sync::Arc<std::sync::Mutex<std::net::TcpListener>>>,
}

impl Listener {

    pub fn new(port: u16, start_receiver: std::sync::mpsc::Receiver<bool>, session_streams_transmitter: std::sync::mpsc::Sender<(std::net::TcpStream, std::net::SocketAddr)>) -> Listener {
        
        let self_host: String = format!("127.0.0.1:{}", port);  
        let sock: std::sync::Arc<std::sync::Mutex<std::net::TcpListener>> = 

            std::sync::Arc::new(

                std::sync::Mutex::new(
                
                    std::net::TcpListener::bind(self_host.as_str())
                    .expect("Errore nell inizializzazione del thread listener")));
        
        let sock_clone = std::sync::Arc::clone(&sock);


        let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            start_receiver.recv().unwrap();

            let sock: std::sync::MutexGuard<std::net::TcpListener> = sock_clone.try_lock().unwrap();
            match sock.accept() {
                Ok((socket, addr)) => {
                    let connection = (socket, addr);
                    session_streams_transmitter.send(connection);
                }
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        });

        Listener {
            listener_thread: Some(handle),
            socket: Some(sock),
        }
    }
}