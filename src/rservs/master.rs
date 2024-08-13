pub struct Listener {
    pub listener_thread: std::thread::JoinHandle<()>,
    socket: std::net::TcpListener,
}

impl Listener{

    pub fn new(port: u16, receiver: std::sync::mpsc::Receiver<bool>) -> Listener {

        let self_host = format!("127.0.0.1:{}", port);  
        let handle = std::thread::spawn(move || {
            let msg_received = receiver.recv().unwrap();
            print!("MAIN THREAD IN ESECUZIONE");
        });

        Listener {
            listener_thread: handle,
            socket: std::net::TcpListener::bind(self_host.as_str()).expect("Errore nell inizializzazione del thread listener"),
        }
    }

    fn handle_income_connections(&self) {
        match self.socket.accept() {
            Ok((socket, addr)) => println!("new client: {addr:?}"),
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}