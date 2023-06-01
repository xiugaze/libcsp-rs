use std::collections::VecDeque;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::csp::types::CspPacket;
use crate::csp::{utils, types};
use crate::csp::CspQueue;


pub struct UDPInterface {
    host: IpAddr,
    lport: u16,
    rport: u16,
    // qfifo: Arc<Mutex<VecDeque<types::CspPacket>>>,
    // TODO: somehow need to generalize this type to allow for multiple
    // interfaces in the same list 
}

impl UDPInterface {
    pub fn new(address: &str, port: u16, qfifo: CspQueue) -> Self {
        let address = address.parse::<IpAddr>().unwrap();

        UDPInterface { host: address, lport: port, rport: port }
        // UDPInterface { host: address, lport: port, rport: port , qfifo }
    }

    fn rx_work(&mut self, socket: &UdpSocket, qfifo: CspQueue) {
        let mut buf: [u8; 256] = [0; 256];
        let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
        // buffer has data
        //
        println!("Message from {src_addr}: ");
        utils::dump_buffer(&buf, len);

        let packet = CspPacket::new(len, buf);
    }

    pub fn rx_loop(&mut self, qfifo: CspQueue) {
        // tuple of (IpAddr, u16) implements ToSocketAddrs
        let socket = UdpSocket::bind( (self.host, self.rport) ).expect("Error: Can't create socket");

        loop {
            self.rx_work(&socket, qfifo.clone())
        }
    }

}

impl types::CspInterface for UDPInterface {
    fn nexthop(&self) {
        unimplemented!();
    }
    fn start_thread(&mut self, iface: &mut Arc<Mutex<Box<UDPInterface>>> , qfifo: CspQueue) {
        
    }
}
