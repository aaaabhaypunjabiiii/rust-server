use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::os::windows::io::AsRawSocket;
use std::net::Shutdown;

pub struct Server{
    pub ip: String,
    pub listener: Option<TcpListener>,
    pub streams: Vec<TcpStream>
}

impl Server {
    pub fn open_connection(&mut self) -> std::io::Result<()>{
        self.listener = Some(TcpListener::bind(&self.ip).expect("Failed to connect"));
        println!("Server listening on port 8080...");
        return Ok(());
    }

    pub fn listen_for_connections(&mut self){
        for stream in self.listener.as_ref().expect("Listener isn't initialized").incoming(){
            match stream {
                Ok(socket) => {
                    println!("New client connected: {}", socket.peer_addr().unwrap());
                    self.streams.push(socket);
                }
                Err(_) => {
                    println!("Error accepting connection");
                }
            }
        }
    }

    pub fn send_response(&self){
        for mut stream in &self.streams {
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello");
        }
    }
    
    
    pub fn close_connection_stream(&self, stream_to_close: &TcpStream){
        if self.streams.iter().any(|i| i.as_raw_socket() == stream_to_close.as_raw_socket()) {
            stream_to_close.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
    }
}