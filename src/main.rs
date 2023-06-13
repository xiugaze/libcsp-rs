pub mod csp;
use std::fs::read;

use csp::{Csp, InterfaceType, types::CspPacket, CspId};

use crate::csp::utils::test_buffer;

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
    csp.add_interface("loopback");
    csp.add_interface("drain");

    let buffer_1: [u8; 256] = test_buffer();
    let buffer_2: [u8; 256] = test_buffer();
    let buffer_3: [u8; 256] = test_buffer();

    let packet_1 = CspPacket::new(
        256, 
        buffer_1, 
        CspId::default()
    );

    let packet_2 = CspPacket::new(
        256, 
        buffer_2, 
        CspId::default()
    );

    let packet_3 = CspPacket::new(
        256, 
        buffer_2, 
        CspId::default()
    );

    csp.send_from_list(0, packet_1);
    csp.send_from_list(0, packet_2);
    csp.send_from_list(1, packet_3);

    println!("{:?}", csp.qfifo.lock());

    let packet = csp.read();
    println!("{}", packet);
    let packet = csp.read();
    println!("{}", packet);
}
