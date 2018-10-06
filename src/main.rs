use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::string::*;

mod secure;
mod monitor;

fn main() {
    run();
}

fn run(){
    let args: Vec<String> = 
        env::args().collect::<Vec<String>>();

    if secure::debug_check() {
        println!("No Used Debbugger");
        return 
    }
    else if args.len() < 2 {
        
        match read_watch_list() {
            Ok(_s) => {}
            Err(_why) => {
                //println!("{}", why.to_string())
                puts_help();
            }
        };
    }
    else if args[1] != "watch_list" { // コマンドラインからの読み込み
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

    let scan = std::io::stdin();

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