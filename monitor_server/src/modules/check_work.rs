extern crate rand;

//use std::{fs, mem};
//use std::io::{BufWriter, Write};
//use std::io::{BufReader, Read};
use rand::Rng;

use super::*;

pub fn work(work: String) {
    let message = work.to_string();
    
    match &*message {
        "[FIRST_CON]" => {
        
            let id: String;

            id = rand::thread_rng()
                .gen_ascii_chars()
                .take(32)
                .collect::<String>();

            let line: String = 
                format!("{}{}", "[ + ] ".to_string(), id);

            println!("{}", line);
            
        }
        _ => {
        }
    }
}
