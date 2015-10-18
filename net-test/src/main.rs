use std::net::{TcpListener, TcpStream};
use std::thread;

// traits
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    loop {
        // clear out the buffer so we don't send garbage
        let mut buf = [0; 2048];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(len) => {
            },
        };

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
