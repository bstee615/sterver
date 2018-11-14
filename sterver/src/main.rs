use std::io;
use std::net::TcpListener;

mod sterver;
mod request;

use sterver::handle_client;
use std::env;
use std::collections::HashMap;

struct Arg {
    pub string_val: String,
    pub bool_val: bool,
}

impl Arg {
    fn from_string(s: String) -> Self {
        Arg { string_val: s.to_string(), bool_val: false }
    }
    
    fn from_bool(b: bool) -> Self {
        Arg { string_val: String::new(), bool_val: b }
    }
}

fn getopts() -> HashMap<String, Arg> {
    let args_with_parameters = vec!(String::from("r"));
    let mut args = env::args();
    let mut args_map = HashMap::new();
    loop {
        match args.next() {
            Some(cur_arg) => {
                let cur_arg_name = cur_arg[1..].to_string();
                if cur_arg.starts_with("-") {
                    match args_with_parameters.iter().find(|&&ref a| a == &cur_arg_name) {
                        Some(arg_name) => match args.next() {
                            Some(arg_val) => {
                                args_map.insert(arg_name.to_string(), Arg::from_string(arg_val));
                            },
                            None => {
                                args_map.insert(arg_name.to_string(), Arg::from_bool(true));
                            },
                        },
                        None => println!("No argument found: {}", cur_arg_name),
                    }
                }
            },
            None => { break }
        }
    }
    args_map
}

fn main() -> io::Result<()> {
    let args = getopts();
    let root = &args.get(&String::from("r")).unwrap().string_val;
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address)?;

    println!("Accepting connections at address {addr} from root {root}. Press Ctrl+C to quit.", addr=address, root=root);
    for stream in listener.incoming() {
        let bytes = handle_client(&mut stream?, &root)?;
        println!("Wrote {numbytes} bytes to {addr}", numbytes=bytes, addr=address);
    }
    Ok(())
}
