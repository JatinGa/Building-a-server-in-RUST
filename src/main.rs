use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("New connection: {}", stream.peer_addr().unwrap());
        println!("New connection established");
    }
}