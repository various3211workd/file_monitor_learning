use std::thread;
use std::net::*;
use std::fs;
use std::io::*;

use super::*;

/*
  run function
  
  return None
*/
pub fn run() {
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
}

/*
handle_connection function

@param stream

return None
*/
fn handle_connection(stream: TcpStream) {
  stream.set_nonblocking(true).expect("set_nonblocking call failed");

  let addr = stream.peer_addr().unwrap();
  let message = network::readFunc(stream).to_string();

  match &*message {
    _ => {
      println!("{} -> {}", addr, message);
      putfile(addr.to_string(), message);
    }
  }
}

// Puts Infomation on UsersFile
fn putfile(addr: String, message: String) {

  let filename = addr.split(":").collect::<Vec<&str>>();

  let mut f = BufWriter::new(
    fs::OpenOptions::new().write(true).create(true).append(true).open(format!("{}{}", "log/".to_string(), filename[0])).expect("[Error] log folder not found"));

  f.write(format!("{}{}", message, "\n".to_string()).as_bytes()).unwrap();
}