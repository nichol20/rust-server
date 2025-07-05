use std::collections::VecDeque;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rust_server::ThreadPool;

unsafe extern "C" {
    fn add_ints(a: i64, b: i64) -> i64;
    fn sub_ints(a: i64, b: i64) -> i64;
    fn mul_ints(a: i64, b: i64) -> i64;
    fn div_ints(a: i64, b: i64, result: *mut i64) -> bool;
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Shared user storage (thread-safe)
    let users = Arc::new(Mutex::new(VecDeque::<User>::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let users = Arc::clone(&users);

        pool.execute(|| {
            handle_connection(stream, users);
        });
    }
}

fn handle_connection(mut stream: TcpStream, users: Arc<Mutex<VecDeque<User>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    let (path, query) = path.split_once('?').unwrap_or((path, ""));

    let response = match (method, path) {
        ("GET", "/") => serve_static("index.html", 200),
        ("GET", "/users") => get_users(&users, query),
        ("GET", "/error") => serve_static("non-existent file.html", 200),
        ("GET", "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            serve_static("index.html", 200)
        }
        ("GET", "/api/hello") => serve_json(r#"{"message": "Hello, world!"}"#),
        ("POST", "/users") => {
            let body = request.split("\r\n\r\n").nth(1).unwrap_or(""); // Extract request body
            post_user(body, &users)
        }
        ("POST", "/math") => {
            let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
            post_math(body)
        }
        _ => serve_static("404.html", 404),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_static(filename: &str, status_code: u16) -> String {
    let message = match status_code {
        200 => "OK",
        404 => "NOT FOUND",
        500 => "INTERNAL SERVER ERROR",
        _ => "",
    };

    match fs::read_to_string(filename) {
        Ok(contents) => format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/html;\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            message,
            contents.len(),
            contents
        ),
        Err(_) => serve_static("500.html", 500),
    }
}

fn serve_json(json: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        json.len(),
        json
    )
}

// Handle POST /users
fn post_user(body: &str, users: &Arc<Mutex<VecDeque<User>>>) -> String {
    if let Some((name, age)) = parse_json_user(body) {
        let mut users_lock = users.lock().unwrap();
        users_lock.push_back(User { name, age });

        return "HTTP/1.1 201 CREATED\r\nContent-Length: 0\r\n\r\n".to_string();
    }
    "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n".to_string()
}

// Handle GET /users
fn get_users(users: &Arc<Mutex<VecDeque<User>>>, query: &str) -> String {
    let mut name_query: Option<&str> = None;
    let mut age_query: Option<u8> = None;

    for pair in query.split('&') {
        let mut key_value = pair.split('=');
        if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
            match key {
                "name" => name_query = Some(value),
                "age" => age_query = value.parse().ok(),
                _ => {}
            }
        }
    }

    let users_lock = users.lock().unwrap();
    let filtered_users: Vec<String> = users_lock
        .iter()
        .filter(|user| {
            name_query.map_or(true, |name| user.name.contains(name))
                && age_query.map_or(true, |age| user.age == age)
        })
        .map(|user| format!(r#"{{"name":"{}","age":{}}}"#, user.name, user.age))
        .collect();
    drop(users_lock); // Explicitly unlock before calling `serve_json`

    let json_response = format!("[{}]", filtered_users.join(","));
    serve_json(&json_response)
}

// Simple JSON parser
fn parse_json_user(body: &str) -> Option<(String, u8)> {
    let body = body.trim_matches(char::from(0)).trim();
    if body.starts_with('{') && body.ends_with('}') {
        let body = &body[1..body.len() - 1]; // Remove { and }
        let mut name = String::new();
        let mut age = None;

        for pair in body.split(',') {
            let mut key_value = pair.split(':').map(str::trim);
            let key = key_value.next()?.trim_matches('"');
            let value = key_value.next()?.trim_matches('"');

            match key {
                "name" => name = value.to_string(),
                "age" => age = value.parse().ok(),
                _ => {}
            }
        }

        if !name.is_empty() && age.is_some() {
            return Some((name, age.unwrap()));
        }
    }

    None
}

fn post_math(body: &str) -> String {
    if let Some((operator, arg1, arg2)) = parse_json_math(body) {
        let result = match operator.as_str() {
            "+" => unsafe { add_ints(arg1, arg2) },
            "-" => unsafe { sub_ints(arg1, arg2) },
            "*" => unsafe { mul_ints(arg1, arg2) },
            "/" => unsafe {
                let mut q = 0i64;
                let ok = div_ints(arg1, arg2, &mut q);
                if !ok {
                    return "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n".to_string();
                }
                q
            },
            _ => return "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n".to_string(),
        };

        let response = format!(
            r#"{{"result":{},"expression":"{} {} {} = {}"}}"#,
            result, arg1, operator, arg2, result
        );
        return serve_json(&response);
    }
    "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 0\r\n\r\n".to_string()
}

fn parse_json_math(body: &str) -> Option<(String, i64, i64)> {
    let body = body.trim_matches(char::from(0)).trim();
    if body.starts_with('{') && body.ends_with('}') {
        let body = &body[1..body.len() - 1]; // Remove { and }
        let mut operator = String::new();
        let mut arg1 = None;
        let mut arg2 = None;

        for pair in body.split(',') {
            let mut key_value = pair.split(':').map(str::trim);
            let key = key_value.next()?.trim_matches('"');
            let value = key_value.next()?.trim_matches('"');

            match key {
                "operator" => operator = value.to_string(),
                "arg1" => arg1 = value.parse().ok(),
                "arg2" => arg2 = value.parse().ok(),
                _ => {}
            }
        }

        if !operator.is_empty() && arg1.is_some() && arg2.is_some() {
            return Some((operator, arg1.unwrap(), arg2.unwrap()));
        }
    }

    None
}
