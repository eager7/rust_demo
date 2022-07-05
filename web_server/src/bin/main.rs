use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = web_server::ThreadPool::new(4);
    for stream in listener.incoming().take(1) {
        let stream = stream.unwrap();
        pool.exec(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    println!("read size:{}", n);
    println!(
        "recv message:-------\n{}",
        String::from_utf8_lossy(&buffer[..])
    );

    let (status, file) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        (
            "HTTP/1.1 200 OK",
            "/Users/chandlerpan/Documents/GitHub/rust_demo/web_server/src/hello.html",
        )
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK",
            "/Users/chandlerpan/Documents/GitHub/rust_demo/web_server/src/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            "/Users/chandlerpan/Documents/GitHub/rust_demo/web_server/src/404.html",
        )
    };
    let response = fs::read_to_string(file).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        response.len(),
        response
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
