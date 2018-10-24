extern crate monitor_server;

use std::io::*;
use std::net::*;
use rand::Rng;
use std::thread;
use std::{fs, mem};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

fn handle_connection(stream: TcpStream) {
    let mut stream = stream;
    stream.set_nonblocking(true).expect("set_nonblocking call failed");
    let addr = stream.peer_addr().unwrap();

    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
    let line: String = 
        String::from_utf8(buf.to_vec()).unwrap();
    
    let message = line.to_string();
    
    match &*message {
        "[FIRST_CON]" => {
        
            let id: String;

            id = rand::thread_rng()
                .gen_ascii_chars()
                .take(32)
                .collect::<String>();

            let line: String = 
                format!("{}{}", "[ + ] ".to_string(), id);

            let bytes: &[u8] = line.as_bytes();
            println!("{}", line);
            stream.write(bytes);
        }
        _ => {
            println!("{} -> {}", addr, message);
            putfile(addr.to_string(), message);
        }
    }
}

fn putfile(addr: String, message: String) {

    let filename = addr.split(":").collect::<Vec<&str>>();

    let mut f = BufWriter::new(
        fs::OpenOptions::new().write(true).create(true).append(true).open(format!("{}{}", "log/".to_string(), filename[0])).unwrap());

    //let mut f = BufWriter::new(
    //    fs::File::create(format!("{}{}", "log/".to_string(), filename[0])).unwrap());

    f.write(format!("{}{}", message, "\n".to_string()).as_bytes()).unwrap();
}

fn run() -> Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 12749)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    thread::spawn(move || {
                        handle_connection(s);
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }

    }
    Ok(())
}
