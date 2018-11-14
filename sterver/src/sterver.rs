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

fn get_file_contents(path: &String, root: &String) -> Result<String, ErrorKind> {
    match fs::read_to_string(Path::new(&format!("{}{}", root, path))) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.kind()),
    }
}

fn http_response_status_line(code: i32, reason: &'static str) -> String {
    format!("HTTP/1.1 {} {}\r\n\r\n", code, reason)
}

fn get_http_response(req: &HttpRequest, root: &String) -> String {
    println!("{}", req);

    if !req.is_valid() {
        http_response_status_line(400, "Bad Request")
    }
    else {
        // println!("{}", http_response_from_contents!(&req.path));
        match get_file_contents(&req.path, &root) {
            Ok(response) => format!("{}{}", http_response_status_line(200, "OK"), response),
            Err(ErrorKind::NotFound) => http_response_status_line(404, "Not Found"),
            Err(_) => http_response_status_line(500, "Internal Server Error"),
        }
    }
}

pub fn handle_client(stream: &mut TcpStream, root: &String) -> io::Result<usize> {
    let buf: TcpBuffer = get_bytes_until_blank_line(&stream);
    let message = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(msg) => return Err(io::Error::new(ErrorKind::InvalidData, msg)),
    };

    let req = HttpRequest::from_str(&message);
    match req {
        Some(unpacked_req) => stream.write(get_http_response(&unpacked_req, &root).as_bytes()),
        None => return Err(io::Error::new(ErrorKind::InvalidData, "Error constructing HTTP request object")),
    }
}
