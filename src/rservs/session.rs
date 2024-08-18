pub struct Client {
    pub session_thread: Option<std::thread::JoinHandle<()>>,
    pub stream: Option<std::sync::Arc<std::sync::Mutex<std::net::TcpStream>>>,
}

impl Client {

    pub fn new(tcp_stream: std::net::TcpStream, start_receiver: std::sync::mpsc::Receiver<bool>) -> Client {
        let stream: std::sync::Arc<std::sync::Mutex<std::net::TcpStream>> = std::sync::Arc::new(std::sync::Mutex::new(tcp_stream));

        let stream_clone = std::sync::Arc::clone(&stream);

        let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            start_receiver.recv().unwrap();

            let stream = stream_clone.lock().unwrap();
            println!("[+] Spawnato un nuovo client, CONNESSIONE A : {}", stream.peer_addr().unwrap());
        });

        Client {
            session_thread: Some(handle),
            stream: Some(stream),
        }
    }
}