use std::io::{self, Write, Error, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use structopt::StructOpt;
mod network;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "localhost")]
    address: String,
    #[structopt(short, long, default_value = "3334")]
    port: String
}

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
        println!("\nReceived from {}: {}", addr, received_text);
    }
    Ok(())
}

fn send_message_to_nodes(message: String, my_address: &String) -> Result<(), Error> {
    let mut active_addresses = network::fetch_active_addresses();
    active_addresses.retain(|address| address != my_address.as_str());
    // println!("We are sending '{}' to these nodes {:?}", message, active_addresses);
    let mut errors = vec![];

    for address in active_addresses {
        match TcpStream::connect(&address) {
            Ok(mut stream) => {
                // Send message
                if let Err(e) = stream.write_all(message.as_bytes()) {
                    println!("Failed to send message to {}: {}", address, e);
                    errors.push(e);
                } else {
                    // println!("Sent!");
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
    let opt = Opt::from_args();
    let my_address = format!("{}:{}", opt.address, opt.port); 
    let listener = TcpListener::bind(&my_address)?;
    println!("Server listening on address: {}", &my_address);

    // Spawn a separate thread to handle console input
    let handle = thread::spawn(move || {
        loop {
            let mut input = String::new();
            print!("Enter message to send: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            send_message_to_nodes(String::from(input.trim()), &my_address).expect("Failed to send message to nodes");
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
