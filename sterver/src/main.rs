use std::io;
use std::net::TcpListener;

mod sterver;
mod request;

use sterver::handle_client;

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address)?;

    println!("Accepting connections at address {addr}. Press Ctrl+C to quit.", addr=address);
    for stream in listener.incoming() {
        let bytes = handle_client(stream?)?;
        println!("Wrote {numbytes} bytes to {addr}", numbytes=bytes, addr=address);
    }
    Ok(())
}
