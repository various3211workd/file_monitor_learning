extern crate monitor_client;

use std::io::*;
use std::fs::*;
use std::env;
use std::thread;

use monitor_client::modules::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

#[warn(unused_must_use)]
fn run() -> Result<()> {
    
    match session_check::check(){
        Ok(_) => { }
        Err(_) => { }
    }

    let args: Vec<String> = 
        env::args().collect::<Vec<String>>();

    if secure::debug_check() {
        println!("No Used Debbugger");
    }
    else if args.len() < 2 {
        
        match read_watch_list() {
            Ok(_) => {}
            Err(_) => {
                puts_help();
            }
        };
    }
    else { // コマンドラインからの読み込み
        for line in args {
            println!("[ * ] starting {} monitor", 
                line);
            let mut _folder_name: String = String::new();
            _folder_name = line.to_string();
            let _child = thread::spawn(move || {
                monitor::run(_folder_name);
            });
        }
    }

    let scan = stdin();

    loop{
        let mut line = String::new();
        let _ = scan.read_line(&mut line);
        
        let vec: Vec<&str> = 
            line.split_whitespace().collect();

        match vec[0] {
            "exit" => break,
            _ => {
                puts_help();
            }
        }
    }
    Ok(())
}

fn read_watch_list() -> std::io::Result<()> {

    let file = File::open("watch_list")?;
    let buf_reader = BufReader::new(file);
    for _folder_name in buf_reader.lines() {
        let _child = thread::spawn(move || {
            monitor::run(_folder_name
                .expect("Failed to read line"));
        });
    }
    Ok(())
}

fn puts_help()
{
    println!("
Name:
    File_Monitor

Overview:
    遠隔ファイル操作監視システム(仮)

syntax:
    ./file_monitor [Directory] [Directory] ..

    # Read 'watch_list' file
    ./file_monitor

Description:
    None

Related links:
    None

Annotation:
    None
");
}