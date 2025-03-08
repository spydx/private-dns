use std::net::UdpSocket;

mod dns;
use dns::response::ResponseBuilder;
use dns::{FromBytes, ToBytes};
use dns::header::Header;
use dns::question::Question;

fn main() {

    let address = "0.0.0.0";
    let port = "1053";
    let socket = UdpSocket::bind(format!("{}:{}", address, port)).expect("Cound not bind to port");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = [];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

// Prints ascii code chars if present:
// https://www.ascii-code.com/
// chars between 31 - 127 in the table.
fn debug_print_bytes(buf: &[u8]) {
    for (i, chunk) in buf.chunks(16).enumerate() {
        print!("{:08x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }
        print!("  ");
        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}


