
#![allow(dead_code)]
// TODO: For unimplemented!()
#![allow(unreachable_code)]


use std::{collections::VecDeque, sync::{Arc, Mutex}, io, rc::Rc};

use self::{
    router::Router, 
    connection::CspConnection,
    types::CspPacket,
    interfaces::{
        NextHop,
        //if_udp::UdpInterface, CspInterfaceState,
        if_loopback::{self, LoopbackInterface}
    }, qfifo::CspQfifo,
};

pub mod tests;
pub mod utils;
pub mod interfaces;
pub mod buffer;
pub mod types;
pub mod router;
pub mod connection;
pub mod qfifo;

pub type InterfaceList = VecDeque<Arc<dyn NextHop>>;

pub struct Csp {
    pub qfifo: Arc<Mutex<CspQfifo>>,
    pub connection_pool: Vec<CspConnection>,
    pub interfaces: InterfaceList,
    pub num_interfaces: usize,
    router: router::Router,
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
        Csp {
            qfifo: Arc::clone(&qfifo), 
            connection_pool: Vec::new(),
            interfaces: VecDeque::new(),
            num_interfaces: 0,
            router: Router::new(Arc::clone(&qfifo)),
        }
    }
}

impl Csp {
    pub fn add_interface(&mut self, iface_type: &str) {
        let qfifo = Arc::clone(&self.qfifo);
        let iface = match iface_type {
            // TODO: Take in an Arc to self, and pass it in
            "loopback" => LoopbackInterface::init(&qfifo, self.num_interfaces),
            _ => panic!("Error: invalid interface name (may not exist)"),
        };
        self.interfaces.push_back(Arc::new(iface));
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
}

