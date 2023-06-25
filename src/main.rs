use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:9000").unwrap();

    for stream in listner.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
}
