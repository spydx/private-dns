use std::net::UdpSocket;

mod dns;
use dns::response::ResponseBuilder;
use dns::{FromBytes, ToBytes};
use dns::header::Header;
use dns::question::Question;

fn main() {
    println!("Hello, world!");
    println!("I am Private DNS");

    let address = "0.0.0.0";
    let port = "1053";
    let socket = UdpSocket::bind(format!("{}:{}", address, port)).expect("Cound not bind to port");

    let mut buf = [0; 512];

    println!("> listening on port: {}", port);

    loop {
        let (len, addr) = socket.recv_from(&mut buf).expect("Could not RX data");

        println!("\nReceived query from {} with length {} bytes", addr, len);
        println!("\n### DNS Query: ###");
        debug_print_bytes(&buf[..len]);

        println!("\n### HEADER ###");
        let header = Header::from_bytes(&buf[..Header::DNS_HEADER_SIZE])
            .expect("Could not parse DNS header");
        println!("\n{:?}", header);

        println!("\n### QUESTION: ###");
        debug_print_bytes(&buf[Header::DNS_HEADER_SIZE..len]);
        println!();

        let question = Question::from_bytes(&buf[Header::DNS_HEADER_SIZE..len])
            .expect("Could not parse DNS question");
        println!("\n{:?}", question);

        let response = ResponseBuilder::new()
                    .add_header(header)
                    .add_question(question)
                    .build();

        let send_buffer = &response.to_bytes();

        socket.send_to(&send_buffer, addr)
                .expect("Cound not send");
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


