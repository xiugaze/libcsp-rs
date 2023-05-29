use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use crate::csp::types::CspPacket;
use crate::csp::{utils, types};

pub trait CspInterface {
    fn nexthop(&self);
}

pub struct UDPInterface {
    host: IpAddr,
    lport: u16,
    rport: u16,
    // TODO: somehow need to generalize this type to allow for multiple
    // interfaces in the same list 
}

impl UDPInterface {
    pub fn init(address: &str, port: u16) -> Self {
        let address = address.parse::<IpAddr>().unwrap();
        UDPInterface { host: address, lport: port, rport: port }
    }

    pub fn rx_loop(&mut self) {
        let mut connection = false;
        // let mut socket: UdpSocket = None;

        while !connection {
            // tuple of (IpAddr, u16) implements ToSocketAddrs
            let mut connected = UdpSocket::bind( (self.host, self.rport) ).expect("Error: Can't create socket");
            connection = !connection;
        }

        // let's figure out the buffers
        let mut buf: Vec<u8> = Vec::new();

        loop {
            let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
            println!("Message from {src_addr}: ");
            utils::dump_buffer(&buf, len);
            // pub struct CspPacket {
            //     length: u16,
            //     // id: CspID,
            //     // next: &CspPacket
            //     header: [u8; 8],
            //     data: CspData,
            // }
            // let packet = CspPacket {
            //     length: len as u16, 
            //     header: buf[0..7].try_into().unwrap(),
            //     data: 
            // };

        }
        // pack this up and send to router
    }
}

impl CspInterface for UDPInterface {
    fn nexthop(&self) {
        unimplemented!();
    }
}
