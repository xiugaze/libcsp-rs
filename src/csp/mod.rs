
#![allow(dead_code)]
// TODO: For unimplemented!()
#![allow(unreachable_code)]


use std::{collections::VecDeque, sync::{Arc, Mutex}, io};

use self::{
    router::Router, 
    connection::CspConnection,
    types::CspPacket,
    interfaces::{
        NextHop,
        if_udp::UdpInterface, CspInterfaceState,
    }, qfifo::CspQfifo,
};

pub mod utils;
pub mod interfaces;
pub mod buffer;
pub mod types;
pub mod router;
pub mod connection;
pub mod qfifo;


// these are going to be architecture specific, use feature guards??
fn router_start() -> u32 { !unimplemented!() }
fn server_start() -> u32 { !unimplemented!() }
fn client_start() -> u32 { !unimplemented!() }

//pub type CspQueue = Arc<Mutex<VecDeque<CspPacket>>>;
pub type InterfaceList = VecDeque<Box<dyn NextHop>>;

pub struct Csp {
    qfifo: CspQfifo,
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
        let qfifo = CspQfifo::new();
        Csp {
            qfifo, 
            connection_pool: Vec::new(),
            interfaces: VecDeque::new(),
            num_interfaces: 0,
            router: Router::new(Arc::clone(&qfifo)),
        }
    }
}

impl Csp {
    pub fn add_interface(&mut self, iface: InterfaceType) {
        let qfifo = Arc::clone(&self.qfifo);
        // TODO: Get rid of the InterfaceType enum, figure something else out
        let csp_interface = Box::from( match iface {
            InterfaceType::Udp => UdpInterface::from(
                "127.0.0.1", 8080, qfifo, CspInterfaceState {name: String::from("UDP"), ..Default::default()})
        });
        self.interfaces.push_back(csp_interface);
        self.num_interfaces += 1;
    }

    // pub fn router_start(&mut self) {
    //     self.router.start(Router::route_work);
    // }

    pub fn send_direct(iface: Box<dyn NextHop>, index: usize, via: u16, packet: CspPacket, from_me: bool) -> io::Result<usize>{
        iface.nexthop(packet)
    }

    pub fn read(&self) -> CspPacket{
        self.qfifo.lock()
            .unwrap()
            .pop_front()
            .unwrap()
    }
}

