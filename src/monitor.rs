extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

pub fn run(folder_name: &str){
    let (tx, rx) = channel();
    
    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(folder_name, 
        RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie: _}) => {
                println!("[ * ] {:?} -> {:?}", op, path);
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
