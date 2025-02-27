use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;

struct Server{
    ip: String,
    listener: Option<TcpListener>
}

impl Server {
    fn open_connection(&mut self) -> std::io::Result<()>{
        self.listener = Some(TcpListener::bind(&self.ip).expect("Failed to connect"));
        println!("Server listening on port 8080...");
        return Ok(());
    }

    fn listen_for_connections(&self){
        for stream in self.listener.as_ref().expect("Listener isn't initialized").incoming(){
            match stream {
                Ok(ref socket) => {
                    println!("New client connected: {}", socket.peer_addr().unwrap());
                    let response = "Received your message!"; 
                    stream.unwrap().write_all(response.as_bytes()).expect("Failed to send");            
                }
                Err(_) => {
                    println!("Error accepting connection");
                }
            }            
        }
    }

    // fn close_connection(&self){
    // }
}
fn main(){
    let mut server = Server {
        ip: String::from("127.0.0.1:8080"),
        listener: None
    };

    let _ = server.open_connection();
    let _ = server.listen_for_connections();

}