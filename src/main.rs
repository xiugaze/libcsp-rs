use std::net::UdpSocket;



pub mod csp;
use csp::{utils, Csp, interfaces::if_udp::UDPInterface, InterfaceType};

fn main() {
   //  let mut socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
   //
   //  let mut buf = [0; 1024];
   // // let mut buf: Vec<u8> = Vec::new();
   //
   //  loop {
   //      let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
   //      //let message = String::from_utf8_lossy(&buf[..len]);
   //
   //      println!("Message from {src_addr}: ");
   //      utils::dump_buffer(&buf, len);
   //  }
   //
    
    let mut csp = Csp::default();
    csp.add_interface(InterfaceType::Udp);
    
}


