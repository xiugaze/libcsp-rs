use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::csp::types::CspPacket;
use crate::csp::{utils, types, CspId};
use crate::csp::CspQueue;
use crate::csp::interfaces::{
    CspInterfaceState,
    NextHop,
};

pub struct UdpInterface {
    iface: Arc<Mutex<CspInterfaceState>>,
    state: Arc<Mutex<UdpState>>,
    rx_thread: Option<JoinHandle<()>>
}

impl UdpInterface {

    /**
       Construct a UDP interface from a hostname and port, a pointer to the global queue
       and a `CspInterfaceState` struct. Fields are stored in an underlying UdpState struct 
       in the `state` field. 
    */
    pub fn from(address: &str, port: u16, qfifo: CspQueue, iface: CspInterfaceState) -> Self {
        let state = UdpState::from(address, port, qfifo);
        UdpInterface {
            iface: Arc::new(Mutex::new(iface)),
            state: Arc::new(Mutex::new(state)),
            rx_thread: None,
        }
    }

    /**
        Start the UDP Receive thread, which will accept incoming connections and 
        store incoming packets in the global queue.
    */
    pub fn start_rx_thread(&mut self) {
        let udp_state = Arc::clone(&self.state); 
        self.rx_thread = Some(thread::spawn(move || {
            let mut udp = udp_state.lock().unwrap();
            udp.rx_loop();
        }));
        
    }
}

pub struct UdpState {
    host: IpAddr,
    lport: u16,
    rport: u16,
    qfifo: CspQueue,
}

impl UdpState {

    /** 
        Construct a `UdpState` struct from a hostname, a port number, and a pointer 
        to the global queue. 
    */
    pub fn from(host: &str, port: u16, qfifo: CspQueue) -> Self {
        let host = host.parse::<IpAddr>().unwrap();
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

        self.push_qfifo(CspPacket::new(len, buf, CspId::default()));
    }

    fn push_qfifo(&mut self, packet: CspPacket) {
        self.qfifo.lock().unwrap().push_back(packet);
    }

    // TODO: Unbounded loops
    pub fn rx_loop(&mut self) {
        // tuple of (IpAddr, u16) implements ToSocketAddrs
        let socket = UdpSocket::bind( (self.host, self.rport) ).expect("Error: Can't create socket");
        for _ in 0..6 {
            self.rx_work(&socket);
        }
    }
}

impl NextHop for UdpInterface {
    fn nexthop(&self, packet: CspPacket) -> io::Result<usize>{
        let state = self.state.lock().unwrap();
        let socket = UdpSocket::bind((state.host, state.lport)).expect("Error: Can't bind to local socket");
        let buf = packet.make_header();
        socket.send(&buf)

    }
    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        let state = Arc::clone(&self.state);
        return state;
    }
}
