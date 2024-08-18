mod rservs;
fn main() {

    let mut sessions: Vec<(rservs::session::Client, std::sync::mpsc::Sender<bool>)> = Vec::new();

    // creating the channels for the Listener thread and the main thread

    // channel to comunicate for the start of the listener thread
    let (start_list_thread_tx, start_list_thread_rx) = std::sync::mpsc::channel::<bool>();
    // channel to comunicate the incoming connections
    let (sessions_tx, sessions_rx) = std::sync::mpsc::channel();  

    // istantiating the Listener thread
    let master_listener_thread = rservs::master::Listener::new(30000, start_list_thread_rx, sessions_tx);
    // starting the thread
    start_list_thread_tx.send(true).unwrap();

    loop {
        let stream = sessions_rx.try_recv();
        
        match stream {
            Ok(stream) => {
                let (session_start_tx, session_start_rx) = std::sync::mpsc::channel::<bool>();
                sessions.push((rservs::session::Client::new(stream, session_start_rx), session_start_tx));

                let (client, tx) = sessions.last().unwrap();
                tx.send(true);
                println!("{}", sessions.len());
            }
            Err(e) => (),
        }
    }
}