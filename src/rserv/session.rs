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
            super::ThreadStatus::Running));


        let stream_clone: std::sync::Arc<std::sync::Mutex<std::net::TcpStream>> = std::sync::Arc::clone(&stream);
        let status_clone = std::sync::Arc::clone(&status);

        let handle: std::thread::JoinHandle<()> = std::thread::Builder::new()
        .name(format!("[CLIENT #{}]", 1).into())
        .spawn(move || {

            println!("[ + ] [CLIENT] new client spawned called {}", std::thread::current().name().unwrap_or("Unnamed"));
            loop {
                // TODO : error handling 
                let result = session_control_receiver.try_recv();
                let change_state = match result {
                    Ok(signal) => signal,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        println!("[ - ] [MAIN] Error, the comunication between [MAIN] and [LISTENER] threads is interrupted");
                        false
                    }
                    _ => false,
                };

                let mut status = status_clone.lock().unwrap();
                match *status {

                    super::ThreadStatus::Blocked => if change_state {*status = super::ThreadStatus::Running},

                    super::ThreadStatus::Running => {
                        
                        Client::handle_request_response(std::sync::Arc::clone(&stream_clone));

                        if change_state {*status = super::ThreadStatus::Blocked};
                    }

                }
            }
        }).expect("[ - ] [MAIN] Error a new [CLIENT] thread can' t be created");
        Client {
            session_thread: handle,
            stream,
            status,
        }
    }

    fn handle_request_response(stream_ref: std::sync::Arc<std::sync::Mutex<std::net::TcpStream>>) {
        let mut stream: std::sync::MutexGuard<std::net::TcpStream> = stream_ref.lock().unwrap();

        let mut buffer = [0; 512];
        match std::io::Read::read(&mut *stream, &mut buffer) {
            Ok(n) => {
                if n > 0 {
                    let received_data = String::from_utf8_lossy(&buffer[..n]);

                    let response = format!(
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/html; charset=UTF-8\r\n\
                        Content-Length: {}\r\n\
                        Connection: close\r\n\r\n\
                        <html>\
                            <head><title>Response</title></head>\
                            <body>\
                                <h1>Hello, World!</h1>\
                                <p>This is a response from the server.</p>\
                            </body>\
                        </html>",
                        "<html><head><title>Response</title></head><body><h1>Hello, World!</h1><p>This is a response from the server.</p></body></html>".len()
                    );
    
                    std::io::Write::write(&mut (*stream), response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Failed to read from the stream: {}", e);
            }
        }
    }
}