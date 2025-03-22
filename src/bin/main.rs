use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use rust_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    let response: String;

    match method {
        "GET" => {
            response = match path {
                "/" => serve_static("index.html"),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    serve_static("index.html")
                }
                "/api/hello" => serve_json(),
                _ => serve_static("404.html"),
            };
        }
        _ => {
            response = "HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\n\r\n".to_string();
        }
    } 

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_static(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        ),
        Err(_) => "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n".to_string(),
    }
}

fn serve_json() -> String {
    let json_response = r#"{"message": "Hello, world!"}"#;
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json_response.len(),
        json_response
    )
}
