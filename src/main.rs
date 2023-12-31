use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use cappy::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:9000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listner.incoming() {
        let stream: TcpStream = stream.unwrap();
        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // previous buffer size of 512 was too small for Chrome
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    println!("---------End of Request------------");

    let get = b"GET / HTTP/1.1\r\n";


    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut html_file = File::open(filename).unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();


    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
