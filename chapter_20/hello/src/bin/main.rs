use hello::{PoolCreationError, ThreadPool};
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), PoolCreationError> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4)?;

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get_index = b"GET / HTTP/1.1\r\n";
    let get_teapot = b"GET /teapot HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_index) {
        ("HTTP/1.1 200 OK", Some("hello.html"))
    } else if buffer.starts_with(get_teapot) {
        ("HTTP/1.1 418 I'M A TEAPOT", None)
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", Some("hello.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", Some("404.html"))
    };

    let contents = if let Some(filename) = filename {
        Some(fs::read_to_string(filename).unwrap())
    } else {
        None
    };

    let response = if let Some(contents) = contents {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        )
    } else {
        format!("{}\r\n\r\n", status_line,)
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
