use std::{collections::VecDeque, sync::{Arc, Mutex}, io, rc::Rc};

use self::{
    router::Router, 
    connection::CspConnection,
    types::{CspPacket, CspResult},
    interfaces::{
        NextHop,
        //if_udp::UdpInterface, CspInterfaceState,
        if_loopback::{self, LoopbackInterface}, if_drain::DrainInterface
    }, qfifo::CspQfifo, port::{CspPort, CspSocket},
};

pub mod tests;
pub mod utils;
pub mod interfaces;
pub mod buffer;
pub mod types;
pub mod router;
pub mod connection;
pub mod qfifo;
pub mod port;

pub type InterfaceList = VecDeque<Arc<dyn NextHop>>;

pub struct Csp {
    pub qfifo: Arc<Mutex<CspQfifo>>,
    pub interfaces: InterfaceList,
    pub num_interfaces: usize,
    router: router::Router,
    pub ports: Arc<Mutex<Vec<CspPort>>>
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CspId {
    priority: u8, 
    flags: u8, 
    source: u16, 
    destination: u16,
    dport: u8, 
    sport: u8,
}

impl CspId {
    pub fn copy_from(&mut self, other: CspId) {
        *self = other;
    }
}

pub enum InterfaceType {
    Udp, 
}

impl Default for Csp {
    fn default() -> Self {
        let qfifo = Arc::new(Mutex::new(CspQfifo::new()));
        let ports = Arc::new(Mutex::new(Vec::new()));
        Csp {
            qfifo: Arc::clone(&qfifo), 
            interfaces: VecDeque::new(),
            num_interfaces: 0,
            ports: Arc::clone(&ports),
            router: Router::new(Arc::clone(&qfifo), Arc::clone(&ports)),
        }
    }
}

impl Csp {
    pub fn add_interface(&mut self, iface_type: &str) {
        let qfifo = Arc::clone(&self.qfifo);
        let iface: Arc<dyn NextHop> = match iface_type {
            // TODO: Take in an Arc to self, and pass it in
            "loopback" => Arc::new(LoopbackInterface::init(&qfifo, self.num_interfaces)),
            "drain" => Arc::new(DrainInterface::new()),
            _ => panic!("Error: invalid interface name (may not exist)"),
        };
        self.interfaces.push_back(iface);
        self.num_interfaces += 1;
    }

    pub fn send_direct(iface: Arc<dyn NextHop>, packet: CspPacket) -> io::Result<usize> {
        iface.nexthop(packet)
    }

    pub fn send_from_list(&mut self, index: usize, packet: CspPacket) -> io::Result<usize> {
        let iface = Arc::clone(&self.interfaces[index]);
        iface.nexthop(packet)
    }

    pub fn read(&self) -> CspPacket {
        let (packet, _) = self.qfifo.lock().unwrap().pop();
        packet
    }
    pub fn route_work(&mut self) {
        self.router.route_work();
    }

    /**
        Binds a socket to a port, and returns the port index. 
    */
    pub fn bind(&mut self, socket: CspSocket) -> CspResult<usize> {
        let port = CspPort {
            state: port::CspPortState::Open,
            socket,
        };
        let mut ports = self.ports.lock().unwrap();
        ports.push(port);
        Ok(ports.len())
    }
}

