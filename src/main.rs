use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filname) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "error.html")
    };

    let index_page = fs::read_to_string(filname).unwrap();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        index_page.len(),
        index_page
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
