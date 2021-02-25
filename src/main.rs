use std::io::prelude::{Write, Read};
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::read_to_string;
use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);

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

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let response = if buffer.starts_with(b"GET / HTTP/1.1") {
        let page = read_to_string("page.html").unwrap();
        format!("HTTP/1.1 200 OK Content-Length: {}\r\n\r\n{}", page.len(), page)

    } else {
        let page = read_to_string("404.html").unwrap();
        format!("HTTP/1.1 404 NOT FOUND Content-Length: {} \r\n\r\n{}", page.len(), page)

    };


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}
