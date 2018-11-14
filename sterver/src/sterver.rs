use std::io;
use std::io::{Write, Read, ErrorKind};
use std::net::TcpStream;
use request::HttpRequest;
use std::vec::Vec;
use std::fs;
use std::path::Path;

type TcpChunk = [u8; 256];
type TcpBuffer = Vec<u8>;

fn get_chunk(mut stream: &TcpStream) -> Option<(TcpChunk, usize)> {
    let mut buf = [0; 256];

    // Try to read from stream and return buf if successful
    match stream.read(&mut buf) {
        Ok(b) => Some((buf, b)),
        Err(_) => {
            println!("Error getting chunk");
            None
        },
    }
}

fn get_bytes_until_blank_line(stream: &TcpStream) -> TcpBuffer {
    let mut bigbuf: TcpBuffer = Vec::new();

    // Loop until entire buffer is recieved
    loop {
        let (buf, size) = match get_chunk(&stream) {
            Some(b) => b,
            None => break,
        };
        bigbuf.extend_from_slice(&buf[..size]);
        if header_terminated(&bigbuf) {
            break;
        }
    };

    bigbuf
}

fn header_terminated(buf: &TcpBuffer) -> bool {
    let cr = '\r' as u8;
    let lf = '\n' as u8;
    let len = buf.len();

    len >= 4 && buf[len-4..len] == [cr, lf, cr, lf]
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

fn get_file_contents(path: &String) -> Result<String, ErrorKind> {
    match fs::read_to_string(Path::new(&path[1..].to_string())) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.kind()),
    }
}

fn http_response_from_contents(contents: String) -> String {
    format!("HTTP/1.1\t200\tOK\r\n\r\n{}", contents)
}

macro_rules! http_response_from_contents {
    ( $path:expr ) => {
        http_response_from_contents(match get_file_contents($path) {
            Ok(s) => s,
            Err(ErrorKind::NotFound) => http_response_not_found!(),
            Err(_) => http_response_server_error!(),
        })
    };
}

macro_rules! http_response_server_error {
    ( ) => {
        String::from("HTTP/1.1 500 Internal Server Error\r\n\r\n")
    };
}

macro_rules! http_response_bad_request {
    ( ) => {
        String::from("HTTP/1.1 400 Bad Request\r\n\r\n")
    };
}

macro_rules! http_response_not_found {
    ( ) => {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };
}

fn get_http_response(req: &HttpRequest) -> String {
    println!("{}", req);

    if !req.is_valid() {
        http_response_bad_request!()
    }
    else {
        // println!("{}", http_response_from_contents!(&req.path));
        http_response_from_contents!(&req.path)
    }
}

pub fn handle_client(stream: &mut TcpStream) -> io::Result<usize> {
    let buf: TcpBuffer = get_bytes_until_blank_line(&stream);
    let message = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(msg) => return Err(io::Error::new(ErrorKind::InvalidData, msg)),
    };

    let req = HttpRequest::from_str(&message);
    match req {
        Some(unpacked_req) => stream.write(get_http_response(&unpacked_req).as_bytes()),
        None => return Err(io::Error::new(ErrorKind::InvalidData, "Error constructing HTTP request object")),
    }
}
