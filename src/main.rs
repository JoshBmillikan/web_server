use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established on {}",stream.peer_addr().unwrap().ip());
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let path = get_filepath(&buffer);
    let (status_line, filename) = if buffer.starts_with(get) {
        (String::from("HTTP/1.1 200 OK\r\n\r\n"), String::from("index.html"))
    } else {
        let s = format!("pages/{}", path);
        (String::from("HTTP/1.1 200 OK\r\n\r\n"), s)
    };
    let mut result = fs::read_to_string(filename);
    if result.is_err() {
        result = fs::read_to_string("404.html");
        let contents = result.unwrap();

        let response = format!("{}{}", status_line, contents);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = result.unwrap();

        let response = format!("{}{}", status_line, contents);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn get_filepath(buf:&[u8; 1024]) -> String {
    let s = &String::from_utf8(buf.to_vec()).unwrap()[5..1024];
    let mut result = String::from("");
    for char in s.chars() {
        if char.eq(&' ') {
            break;
        }
        result += &String::from(char);
    }
    println!("opening {}",result);
    result
}
