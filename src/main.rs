use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::string::*;

mod secure;
mod monitor;

fn run(){
    //let mut args: Vec<String> = 
    //    env::args().collect::<Vec<String>>();
    //let args: Vec<_> = 
    //    env::args().skip(1).collect();
    
    if secure::debug_check() {
        println!("No Used Debbugger");
        return 
    }
    else if args.len() < 2 {
        match read_watch_list() {
            Ok(_s) => {}
            Err(why) => {
                println!("{}", why.to_string())
            }
        };
    }
    else if args[1] != "watch_list" { // コマンドラインからの読み込み
        /*
        args[1..].iter().for_each(|folder_name|{
            println!("[ * ] starting {} monitor", folder_name);
            let _child = thread::spawn(move || {
                monitor::run(folder_name);
            });
        });
        */
        args[1..].iter().for_each(|folder_name|{
            println!("[ * ] starting {} monitor", folder_name);
            monitor::run(folder_name);
        });
    }

    loop{

    }
}

fn main() {
    run();
}

fn read_watch_list() -> std::io::Result<()> {
    let file = File::open("watch_list")?;
    let mut buf_reader = BufReader::new(file);
    let mut folder_name = String::new();

    buf_reader.read_to_string(&mut folder_name)?;
    println!("{}", folder_name);

    monitor::run(&folder_name);
    Ok(())
}