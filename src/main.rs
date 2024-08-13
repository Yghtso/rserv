mod rservs;
fn main() {

    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let master_listener_thread = rservs::master::Listener::new(30000,rx);

    //tx.send(true).unwrap();

    master_listener_thread.listener_thread.join().unwrap();
}