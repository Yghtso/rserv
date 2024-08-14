mod rservs;
fn main() {
    let (start_list_thread_tx, start_list_thread_rx) = std::sync::mpsc::channel::<bool>();
    let (sessions_tx, sessions_rx) = std::sync::mpsc::channel();  
    
    let master_listener_thread = rservs::master::Listener::new(30000, start_list_thread_rx, sessions_tx);
    start_list_thread_tx.send(true).unwrap();
    master_listener_thread.listener_thread.unwrap().join().unwrap();

}