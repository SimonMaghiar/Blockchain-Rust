use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let localhost_ips = vec!["localhost:3334", "localhost:3335", "localhost:3336"];
    let ips_string = localhost_ips.join("\n");
    
    if let Err(err) = stream.write_all(ips_string.as_bytes()) {
        println!("Error writing to stream: {}", err);
    }
    
    println!("Connection closed with {}", stream.peer_addr().unwrap());
    let _ = stream.shutdown(Shutdown::Both);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").expect("Failed to bind to address");
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("Server closed");
}
