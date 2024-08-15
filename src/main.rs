mod rservs;
fn main() {

    let sessions: Vec<rservs::session::Client> = Vec::new();

    // creating the channels for the Listener thread and the main thread

    // channel to comunicate for the start of the listener thread
    let (start_list_thread_tx, start_list_thread_rx) = std::sync::mpsc::channel::<bool>();
    // channel to comunicate the incoming connections
    let (sessions_tx, sessions_rx) = std::sync::mpsc::channel();  


    // istantiating the Listener thread
    let master_listener_thread = rservs::master::Listener::new(30000, start_list_thread_rx, sessions_tx);
    // starting the thread
    start_list_thread_tx.send(true).unwrap();

    

    master_listener_thread.listener_thread.unwrap().join().unwrap();

}