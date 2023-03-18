use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[test]
fn test_handle_connection() {
    // Start a server in a separate thread
    let server_thread = thread::spawn(|| {
        let address = "127.0.0.1:0";
        let listener = TcpListener::bind(address).unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let response = "Hello, world!".to_string();
                    stream.write(response.as_bytes()).unwrap();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    });

    // Connect to the server and send a message
    let address = server_thread.join().unwrap().local_addr().unwrap();
    let mut stream = TcpStream::connect(address).unwrap();
    let message = "Hello, server!".to_string();
    stream.write(message.as_bytes()).unwrap();

    // Read the server's response
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..n]);

    // Verify that the server's response matches what we expected
    assert_eq!(response, "Hello, world!");

    // Clean up the server thread
    drop(server_thread);
}
