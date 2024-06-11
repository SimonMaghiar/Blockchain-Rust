use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;

const SEED_NODE: &str = "localhost:3333";

pub fn fetch_active_addresses() -> Vec<String> {
    let mut active_addresses = Vec::new();

    match TcpStream::connect(SEED_NODE) {
        Ok(mut stream) => {
            // println!("Successfully connected to SEED_NODE");

            let mut buffer = Vec::new();
            match stream.read_to_end(&mut buffer) {
                Ok(_) => {
                    if let Ok(reply) = from_utf8(&buffer) {
                        active_addresses = reply.split('\n').map(|s| s.to_string()).collect();
                    } else {
                        println!("Received non-UTF8 reply from server");
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    active_addresses
}
