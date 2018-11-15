extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

use super::*;

pub fn run(folder_name: String){
    let (tx, rx) = channel();
    
    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(folder_name, 
        RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie: _}) => {
                let line: String = 
                    format!("{}{:?}{:?}",
                        "[ * ] ".to_string(), 
                        op, 
                        path);
                println!("{}", line);
                network::send(line);
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

