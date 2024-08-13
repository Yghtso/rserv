pub struct Listener {
    listener_thread: std::thread::JoinHandle<()>,
    socket: std::net::TcpListener,
}

impl Listener{
    pub fn run(port: u16) -> Listener {

        let self_host = format!("127.0.0.1:{}", port);  
        let handle = std::thread::spawn(move || {
            Listener::handle_income_connections();
        });

        Listener {
            listener_thread: handle,
            socket: std::net::TcpListener::bind(self_host.as_str()).expect("Errore nell inizializzazione del thread listener"),
        }
    }

    fn handle_income_connections() {
        //for stream in .incoming() {
        //    handle_client(stream?);
        //}
    }
}