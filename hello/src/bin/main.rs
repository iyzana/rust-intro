extern crate hello;

use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind(("::1", 7878)).unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        pool.execute(|| handle_connection(stream.unwrap()));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let start = Instant::now();

    let mut request = [0; 512];
    stream.read(&mut request).unwrap();

    let (status, template) = route(&request);

    let filename = format!("templates/{}.html", template);
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("sent response in {:?}", start.elapsed());
}

fn route<'a>(request: &[u8]) -> (&'a str, &'a str) {
    if let Some(request) = String::from_utf8_lossy(request).lines().next() {
        println!("{}", request);
    }

    let root = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if request.starts_with(root) {
        ("200 OK", "hello")
    } else if request.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello")
    } else {
        ("404 NOT FOUND", "404")
    }
}
