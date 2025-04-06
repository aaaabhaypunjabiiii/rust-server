pub mod types;

use crate::types::Server;

fn main(){
    let mut server = Server {
        ip: String::from("127.0.0.1:8080"),
        listener: None,
        streams: Vec::new()
    };

    let _ = server.open_connection();
    let _ = server.listen_for_connections();

}