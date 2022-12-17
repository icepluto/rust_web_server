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
    // println!("Request:{}",String::from_utf8_lossy(&buffer[..]));

    let index_page = fs::read_to_string("hello.html").unwrap();
    // println!("{:?}",page);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        index_page.len(),
        index_page
    );
    // let res_byte = response.as_bytes();
    // println!("{:?}",res_byte);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
