use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
fn main() {
    let tcp_listener = TcpListener::bind("192.168.10.92:1500").unwrap();
    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_reqest: Vec<_> = buf_reader
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    println!("http: {http_reqest:#?}");

    let http_response = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let content_lenght = content.len();
    let response = format!("{http_response}\r\nContent-Length: {content_lenght}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
