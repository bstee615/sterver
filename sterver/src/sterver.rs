use std::io;
use std::str;
use std::io::{Write, Read};
use std::net::TcpStream;
use request::HttpRequest;
use std::vec::Vec;
use std::fs::File;
use std::path::Path;

type TcpBuffer = Vec<u8>;

fn get_bytes(mut stream: &TcpStream) -> TcpBuffer {
    let mut buf: TcpBuffer = vec![0; 256];

    // Try to read from stream and return buf if successful
    match stream.read(&mut buf) {
        Ok(b) => println!("Got {bytes} bytes:", bytes=b),
        Err(_) => println!("Error"),
    }

    let mut newline_pos = 0;
    for i in 0..256 {
        if buf[i] == '\n' as u8 {
            newline_pos = i;
            break;
        }
    }

    if newline_pos == 256 {
        buf
    }
    else {
        buf[..newline_pos+1].to_vec()
    }
}

fn header_terminated(buf: &TcpBuffer) -> bool {
    let nl = '\n' as u8;
    let len = buf.len();

    if len < 2 {
        false
    }
    else {
        let mut ret = false;
        for i in 1..buf.len() {
            // println!("{}", buf[i]);
            if buf[i] == nl && buf[i-1] == nl {
                ret = true;
            }
        }
        ret
    }
}

fn get_bytes_until_blank_line(mut stream: &TcpStream) -> TcpBuffer {
    let mut bigbuf: TcpBuffer = Vec::new();
    while !header_terminated(&bigbuf) {
        bigbuf.append(& mut get_bytes(&stream));
    }

    bigbuf
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

fn get_file_contents(path: &String) -> [u8; 1024] {
    let mut buf = [0; 1024];
    let file = File::open(Path::new(&path));
    match file {
        Ok(mut f) => {f.read(& mut buf);},
        _Error => println!("Error opening file {}", path),
    }
    
    buf
}

fn write_response(mut stream: &TcpStream, req: &HttpRequest) -> io::Result<usize> {
    if !req.is_valid() {
        println!("{}", req);
        write_invalid_request(stream)
    }
    else {
        println!("{}", req);
        stream.write(&get_file_contents(&req.path))
    }
}

fn write_invalid_request(mut stream: &TcpStream) -> io::Result<usize> {
    stream.write(b"Invalid HTTP request.")
}

pub fn handle_client(stream: TcpStream) -> io::Result<usize> {
    let buf: TcpBuffer = get_bytes_until_blank_line(&stream);
    // print_buf(&buf);

    let req = get_http_request(&buf);
    match req {
        Some(unpacked_req) => write_response(&stream, &unpacked_req),
        None => write_invalid_request(&stream),
    }
}
