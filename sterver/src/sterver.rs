use std::io;
use std::str;
use std::io::{Write, Read};
use std::net::TcpStream;
use request::HttpRequest;
use std::vec::Vec;

type TcpBuffer = Vec<u8>;

fn get_bytes(mut stream: &TcpStream) -> TcpBuffer {
    let mut buf: TcpBuffer = vec![0; 256];

    // Try to read from stream and return buf if successful
    match stream.read(&mut buf) {
        Ok(b) => println!("Got {bytes} bytes:", bytes=b),
        Err(_) => println!("Error"),
    }
    
    buf
}

#[allow(dead_code)]
fn print_buf(buf: &TcpBuffer) {
    for i in 0..buf.len() {
        let b = buf[i];
        if b != 0 {
            println!("byte #{}: 0x{:02X}", i, b);
        }
    }
    println!();
}

fn get_http_request(buf: &TcpBuffer) -> Option<HttpRequest> {
    match str::from_utf8(&buf) {
        Ok(s) => HttpRequest::from_str(s),
        Err(_) => {
            println!("Error decoding bytes.");
            None
        }
    }
}

fn write_response(mut stream: &TcpStream, req: &HttpRequest) -> io::Result<usize> {
    if !req.is_valid() {
        println!("{}", req);
        write_invalid_request(stream)
    }
    else {
        println!("{}", req);
        stream.write(b"Valid HTTP request.")
    }
}

fn write_invalid_request(mut stream: &TcpStream) -> io::Result<usize> {
    stream.write(b"Invalid HTTP request.")
}

pub fn handle_client(stream: TcpStream) -> io::Result<usize> {
    let buf: TcpBuffer = get_bytes(&stream);
    // print_buf(&buf);

    let req = get_http_request(&buf);
    match req {
        Some(unpacked_req) => write_response(&stream, &unpacked_req),
        None => write_invalid_request(&stream),
    }
}
