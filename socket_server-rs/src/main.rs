use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const MAX_PAYLOAD: u16 = 16384 + 2048;
const HEADER_SIZE: u16 = 1 + 2 + 2;
pub const MAX_WIRE_SIZE: usize = (MAX_PAYLOAD + HEADER_SIZE) as usize;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8089").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    println!("Success");
}


fn handle_client(
    mut stream: TcpStream,
) {
    loop {
        let mut buf = [0u8; MAX_WIRE_SIZE];
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => {
                break;
            }
            Ok(n) => {
                stream.write_all(&buf[..n]).unwrap();
            }
        }
    }
}

