//use std::env;
use std::fs::File;
//use std::io::prelude::*;
//use std::string::*;
use std::io::{BufWriter, Write};
//use std::io::{BufReader, Read};

use super::*;

pub fn check() -> std::io::Result<()> {
    open_event();

    Ok(())
}

fn open_event() -> std::io::Result<()> {
    let mut _f = match File::open("event.dump"){
        Ok(_) => {

        }
        Err(_) => {
            //first_session();
        }
    };
    Ok(())
}

fn first_session() -> std::io::Result<()> {
    let f = File::create("event.dump").unwrap();

    let message: String = 
        "[FIRST_CON]".to_string();

    network::send(message);
    let message: String = network::read();

    let mut bf = BufWriter::new(f);
    for _ in 0 .. 100 {
        bf.write(message.as_bytes()).unwrap();
    }

    Ok(())
}