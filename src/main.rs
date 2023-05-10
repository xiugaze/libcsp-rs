use std::net::UdpSocket;



//pub mod csp;
pub mod utils;

fn main() {
    let mut socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut buf = [0; 1024];

    loop {
        let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
        //let message = String::from_utf8_lossy(&buf[..len]);

        println!("Message from {src_addr}: ");
        utils::dump_buffer(&buf, len);
    }
}


