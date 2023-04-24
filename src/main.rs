use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};

pub mod csp;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:10").unwrap();

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
    }
    for (i, b) in buffer.iter().enumerate() {
        println!("idx: {:?} = {:?}", i, b);
    }
}


