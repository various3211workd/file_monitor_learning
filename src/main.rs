#[macro_use]
extern crate serde_derive;
use docopt::Docopt;

use std::io::*;
use std::fs::*;
use std::thread;

use std::net::*;
use std::{fs, mem};

use moni::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
  moni
  moni <path>...
  moni -l <port>
  moni (--help | --version)

Options:
  -l, --listen   Listen port
  -h, --help     Show this screen
  -v, --version  Show version
";

#[derive(Deserialize)]
struct Args {
  arg_path: Vec<String>,
  arg_port: String,
  flag_listen: bool,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.flag_listen {
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
  else {
    match args.arg_path.len() {
      // start monitoring on watch file
      0 => {
        let file = match File::open("watch_list") {
          Err(why) => panic!("{}", &why),
          Ok(file) => file,
        };
        let buf_reader = BufReader::new(file);
        for _folder_name in buf_reader.lines() {
          let _child = thread::spawn(move || {
            monitor::run(_folder_name.expect("Failed to read line"));
          });
        }
      }
      // start monitoring on input commandline
      _ => {
        for file_path in args.arg_path {
          let _child = thread::spawn(move || {
            monitor::run(file_path);
          });
        }
      }
    }
  }

  // input commands
  let scan = stdin();
  loop{
    let mut line = String::new();
    let _ = scan.read_line(&mut line);

    let vec: Vec<&str> = 
      line.split_whitespace().collect();

    match vec[0] {
      "exit" => break,
      _ => { },
    }
  }
}

// Server Listen
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
    fs::OpenOptions::new().write(true).create(true).append(true).open(format!("{}{}", "log/".to_string(), filename[0])).unwrap());

  f.write(format!("{}{}", message, "\n".to_string()).as_bytes()).unwrap();
}