use std::env;

mod secure;
mod monitor;

fn main() {
    let args: Vec<String> = 
        env::args().collect::<Vec<String>>();
    
    args[1..].iter().for_each(|folder_name|{
        println!("starting {} monitor", folder_name);
        monitor::file_monitor(folder_name);
    });

    if secure::debug_check() {
        println!("Error");
    }
}
