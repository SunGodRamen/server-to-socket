use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[cfg(test)]
mod app_test;

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => {
                // End of file, break the loop.
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", message);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
