pub struct Client {
    pub session_thread: std::thread::JoinHandle<()>,
    pub stream: std::sync::Arc<std::sync::Mutex<std::net::TcpStream>>,
    pub status: std::sync::Arc<std::sync::Mutex<super::ThreadStatus>>,
}

impl Client {

    pub fn new(tcp_stream: std::net::TcpStream, session_control_receiver: std::sync::mpsc::Receiver<bool>) -> Client {
        let stream = 
            std::sync::Arc::new(
            std::sync::Mutex::new(tcp_stream));

        let status = 
            std::sync::Arc::new(
            std::sync::Mutex::new(
            super::ThreadStatus::Blocked));


        let stream_clone: std::sync::Arc<std::sync::Mutex<std::net::TcpStream>> = std::sync::Arc::clone(&stream);
        let status_clone = std::sync::Arc::clone(&status);

        let handle: std::thread::JoinHandle<()> = std::thread::Builder::new()
        .name(format!("[CLIENT #{}]", 1).into())
        .spawn(move || {
            loop {
                // TODO : error handling 
                let status = status_clone.lock().unwrap();
                match *status {

                    super::ThreadStatus::Blocked => {
                    
                        let result = session_control_receiver.try_recv();
                    
                        match result {
                            Ok(starting) => if starting { break; },
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                println!("[ - ] [MAIN] Error, the comunication between [MAIN] and [LISTENER] threads is interrupted");
                            }
                            _ => (), 
                        }
                    }

                    super::ThreadStatus::Running => {

                        // TODO : implement this
                    }

                }
            }
            let stream: std::sync::MutexGuard<std::net::TcpStream> = stream_clone.lock().unwrap();
        }).expect("[ - ] [MAIN] Error a new [CLIENT] thread can' t be created");

        Client {
            session_thread: handle,
            stream,
            status,
        }
    }
}