use std::io::{self, BufRead, Write, Error, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
mod network;

const NODE_2_ADDRESS: &str = "localhost:3335";

fn receive_message(mut stream: TcpStream, addr: std::net::SocketAddr) -> io::Result<()> {
    // Buffer to store received data
    let mut buffer = [0; 1024];
    loop {
        // Read data from the client
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // Connection closed
            break;
        }
        // Convert received bytes to string
        let received_text = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received from {}: {}", addr, received_text);
    }
    Ok(())
}

fn send_message_to_nodes(message: String) -> Result<(), Error> {
    let mut active_addresses = network::fetch_active_addresses();
    active_addresses.retain(|address| address != NODE_2_ADDRESS);
    println!("We are sending '{}' to these nodes {:?}", message, active_addresses);
    let mut errors = vec![];

    for address in active_addresses {
        match TcpStream::connect(&address) {
            Ok(mut stream) => {
                // Send message
                if let Err(e) = stream.write_all(message.as_bytes()) {
                    println!("Failed to send message to {}: {}", address, e);
                    errors.push(e);
                } else {
                    println!("Sent!");
                    // Optionally receive response from nodes
                }
            }
            Err(e) => {
                println!("Failed to connect to {}: {}", address, e);
                errors.push(e);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to send message to one or more nodes"))
    }
}

fn main() -> io::Result<()> {
    // Start listening on localhost:3335
    let listener = TcpListener::bind(NODE_2_ADDRESS)?;
    println!("Server listening on port 3335...");

    // Spawn a separate thread to handle console input
    let handle = thread::spawn(|| {
        loop {
            let mut input = String::new();
            print!("Enter message to send: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Send message to nodes
            send_message_to_nodes(String::from(input.trim())).expect("Failed to send message to nodes");
        }
    });

    // Accept incoming connections and handle them in separate threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().expect("failed to get peer address");
                thread::spawn(move || {
                    // Handle client connection
                    if let Err(err) = receive_message(stream, addr) {
                        eprintln!("Error handling client: {}", err);
                    }
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }

    // Wait for console input thread to finish (this will never happen in this example)
    if let Err(e) = handle.join() {
        println!("Thread join error: {:?}", e);
    }

    Ok(())
}
