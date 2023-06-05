use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::csp::types::CspPacket;
use crate::csp::{utils, types};
use crate::csp::CspQueue;


pub struct UdpInterface {
    state: Arc<Mutex<UdpState>>,
    thread: Option<JoinHandle<()>>,
}

impl UdpInterface {
    pub fn from(address: &str, port: u16, qfifo: CspQueue) -> Self {
        let state = UdpState::from(address, port, qfifo);
        UdpInterface {
            state: Arc::new(Mutex::new(state)),
            thread: None,
        }
    }

    pub fn start(&mut self) { 
        let udp = Arc::clone(&self.state);
        self.thread = Some(thread::spawn(move || {
            let mut udp = udp.lock().unwrap();
            //let qfifo = Arc::clone(&udp.qfifo);
            udp.rx_loop()
        }));
    }

    pub fn stop_and_wait(&mut self) {
        self.thread.take().unwrap().join().unwrap();
    }
}

pub struct UdpState {
    host: IpAddr,
    lport: u16,
    rport: u16,
    qfifo: CspQueue,
}

impl UdpState {
    pub fn from(host: &str, port: u16, qfifo: CspQueue) -> Self {
        let host = host.parse::<IpAddr>().unwrap();
        let qfifo = Arc::clone(&qfifo);
        UdpState { 
            host, 
            lport: port, 
            rport: port, 
            qfifo,
        }
    }

    fn rx_work(&mut self, socket: &UdpSocket) {
        let mut buf: [u8; 256] = [0; 256];
        let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
        println!("Message from {src_addr}: ");
        utils::dump_buffer(&buf, len);

        self.push_qfifo(CspPacket::new(len, buf));
    }

    fn push_qfifo(&mut self, packet: CspPacket) {
        self.qfifo.lock().unwrap().push_back(packet);
    }

    pub fn rx_loop(&mut self) {
        // tuple of (IpAddr, u16) implements ToSocketAddrs
        let socket = UdpSocket::bind( (self.host, self.rport) ).expect("Error: Can't create socket");
        for _ in 0..6 {
            self.rx_work(&socket);
        }
    }
}

impl types::NextHop for UdpInterface {
    fn nexthop(&self, _via: u16, packet: CspPacket, _from_me: bool) -> io::Result<usize>{
        let state = self.state.lock().unwrap();
        let socket = UdpSocket::bind((state.host, state.lport)).expect("Error: Can't bind to local socket");

        let buf = packet.make_header();
        socket.send(&buf)
    }
}
