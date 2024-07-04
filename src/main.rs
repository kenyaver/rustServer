use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
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

    let http_response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(http_response.as_bytes()).unwrap();
}
