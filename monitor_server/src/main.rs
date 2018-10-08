extern crate monitor_server;

use std::io::*;
use std::net::{TcpListener, TcpStream, SocketAddr};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

fn handle_connection(stream: TcpStream) {
    let mut stream = stream;
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

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

    /*match line {
        "first_session".to_string() => {

        }
        _ => { println!("{}", line); }
    }*/
    println!("{}", line);
}

fn run() -> Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 12749)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_connection(s);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
    Ok(())
}

/*
#[warn(unused_must_use)]
fn run() -> Result<()> {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 12749)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => { stream }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };

            let test: &str = "Test";
            let bytes: &[u8] = test.as_bytes();
            stream.write(bytes);
        }

    Ok(())
}
*/