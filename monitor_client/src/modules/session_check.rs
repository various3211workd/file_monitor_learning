use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::string::*;

use super::*;

pub fn check() -> std::io::Result<()> {
    match open_event() {
        Ok(_) => (),
        Err(e) => {
            first_session();
        }
    }

    Ok(())
}

fn open_event() -> std::io::Result<()> {
    let mut f = match File::open("event.dump"){
        Ok(_) => {

        }
        Err(_) => {
            first_session();
        }
    };
    Ok(())
}

fn first_session() -> std::io::Result<()> {
    let mut f = File::create("event.dump");

    let mut message: String = 
        "[FIRST_CON]".to_string();

    network::send(message);

    Ok(())
}