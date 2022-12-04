use hello::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{env, fs};

fn handle_connection(mut stream: TcpStream) {
    let path = env::current_dir().unwrap();
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/200.html")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/200.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/html/404.html")
    };

    let contents = fs::read_to_string(path.join(filename)).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    println!("Hello, world!");
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
