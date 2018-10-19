use std::fs::*;
use std::io::*;
use std::net::{TcpStream, SocketAddr, Shutdown};

#[warn(unused_must_use)]
pub fn check() -> std::io::Result<()> {
    if metadata("event.dump").is_ok() {
        println!("[ v ] event.dump ok");
    }
    else{
        let f = File::create("event.dump").unwrap();

        let message: String = 
            "[FIRST_CON]".to_string();

        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 12749)),
        ];
        let mut stream = TcpStream::connect(&addrs[..])
            .expect("Couldn't connect to the server...");
        let bytes: &[u8] = message.as_bytes();
        
        loop {
            match stream.write(bytes) {
                Ok(_) => break,
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };
        };

        let mut buf = vec![];
        loop {  // ここで止まる
            match stream.read_to_end(&mut buf) {
                Ok(_) => break,
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };
        };
        let message: String = 
            String::from_utf8(buf.to_vec()).unwrap();
        stream.shutdown(Shutdown::Both)
            .expect("shutdown call failed");

        println!("{}", message);

        let mut bf = BufWriter::new(f);
        for _ in 0 .. 100 {
            bf.write(message.as_bytes()).unwrap();
        }
    }

    Ok(())
}

/*
fn first_session() -> std::io::Result<()> {
    let f = File::create("event.dump").unwrap();

    let message: String = 
        "[FIRST_CON]".to_string();

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 12749)),
    ];
    let mut stream = TcpStream::connect(&addrs[..])
        .expect("Couldn't connect to the server...");
    let bytes: &[u8] = message.as_bytes();
    loop {
        match stream.write(bytes) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };

    let mut buf = vec![];
    loop {  // ここで止まる
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    };
    let message: String = 
        String::from_utf8(buf.to_vec()).unwrap();
    stream.shutdown(Shutdown::Both)
        .expect("shutdown call failed");

    println!("{}", message);

    let mut bf = BufWriter::new(f);
    for _ in 0 .. 100 {
        bf.write(message.as_bytes()).unwrap();
    }

    Ok(())
}
*/