use std::fs::File;
use std::io::{Read, Write};
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

    let mut html_file = File::open("index.html").unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();


    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
