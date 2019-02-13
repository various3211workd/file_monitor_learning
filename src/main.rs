#[macro_use]
extern crate serde_derive;
use docopt::Docopt;

use std::io::*;
use std::fs::*;
use std::thread;

use moni::modules::*;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
  moni
  moni <path>...
  moni -l
  moni (--help | --version)

Options:
  -l, --listen   Server Listen
  -h, --help     Show this screen
  -v, --version  Show version
";

#[derive(Deserialize)]
struct Args {
  arg_path: Vec<String>,
  flag_listen: bool,
}

#[warn(unused_must_use)]
fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.flag_listen {
    println!("[[Listen Monitoring]]");
    listen_server::run();
  }
  else {
    match args.arg_path.len() {
      // start monitoring on watch file
      0 => {
        let file = match File::open("watch_list") {
          Err(_why) => {
            panic!("[Error]Not Found watch_list");
          },
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
}

